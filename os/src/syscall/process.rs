//! Process management syscalls

use crate::config::{MAX_SYSCALL_NUM, PAGE_SIZE};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, get_current_status, get_current_syscall_times, get_current_start_time, current_user_token, current_memory_map, current_memory_unmap, TaskStatus};
use crate::timer::{get_time_us, get_time_ms};
use crate::mm::{translated_byte_buffer, VirtAddr, MapPermission};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let buffers = translated_byte_buffer(current_user_token(), _ts as * const u8, 1);
    let addr = &buffers[0][0] as *const u8 as *mut TimeVal;
    unsafe {
        *addr = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    if _start % PAGE_SIZE != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1
    }
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len).into();
    let permission = MapPermission::from_bits((_port << 1 | 1 << 4) as u8).unwrap();
    current_memory_map(start_va, end_va, permission)
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    if _start % PAGE_SIZE != 0 {
        return -1
    }
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len).into();
    current_memory_unmap(start_va, end_va)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let buffers = translated_byte_buffer(current_user_token(), ti as * const u8, 1);
    let addr = &buffers[0][0] as *const u8 as *mut TaskInfo;
    unsafe {
        *addr = TaskInfo {
            status: get_current_status(),
            syscall_times: get_current_syscall_times(),
            time: get_time_ms() - get_current_start_time(),
        };
    }
    0
}
