# OS Lab5 实验报告

<h4>王哲凡 计93 2019011200</h4>

## 实现功能

- 首先实现用于死锁检测的封装数据结构 `DeadLockDetectStruct`，并实现对应的修改/增加/删除接口和检测接口。
- 在 `ProcessControlBlockInner` 中为锁和信号量分别添加对应的数据结构实例，并增加死锁检测开启标志。
- 在 `enable_deadlock_detect` 系统调用中更新标志，在锁和信号量相关系统调用中进行数据结构的更新和死锁检测。

## 问答题

### 第 1 题

> 在我们的多线程实现中，当主线程 (即 $0$ 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。
> - 需要回收的资源有哪些？
> - 其他线程的 `TaskControlBlock` 可能在哪些位置被引用，分别是否需要回收，为什么？

需要回收的资源包括：
- 每个线程的内核栈；
- 每个线程的任务上下文；
- 整个进程的用户栈等地址空间；
- 整个进程的文件描述符表、线程资源分配器以及所有进程内的锁/信号量/条件变量。

其他线程的 `TaskControlBlock` 可能在以下位置被引用：
- 锁/信号量/条件变量的等待队列中，需要回收，因为可能存在线程还在等待使用；
- 计时条件变量（`TimerCondVar`）中记录任务线程，需要回收，因为可能存在线程此时还处于系统调用 `sleep` 中；
- `TaskManager` 的等待队列中，需要回收，因为可能还有部分线程自身并未结束。

### 第 2 题

> 对比以下两种 `Mutex.unlock` 的实现，二者有什么区别？这些区别可能会导致什么问题？
> ```rust
> impl Mutex for Mutex1 {
>     fn unlock(&self) {
>         let mut mutex_inner = self.inner.exclusive_access();
>         assert!(mutex_inner.locked);
>         mutex_inner.locked = false;
>         if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
>             add_task(waking_task);
>         }
>     }
> }
> 
> impl Mutex for Mutex2 {
>     fn unlock(&self) {
>         let mut mutex_inner = self.inner.exclusive_access();
>         assert!(mutex_inner.locked);
>         if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
>             add_task(waking_task);
>         } else {
>             mutex_inner.locked = false;
>         }
>     }
> }
> ```

区别在于，当等待队列不为空时，前者仍会将 `mutex_inner.locked` 置为 `false`，等待后续修改，后者则会直接将锁的所有权移交给等待时间最久的线程。

这种区别导致，在锁 `unlock()` 后进行任务切换时，如果切换到了等待队列中的其他线程而不是等待时间最长的线程时，前者会将锁的使用权交给这个新调度的线程，而后者会将其阻塞，并直到调度到等待时间最久的线程时才继续执行。

这也就会使得前者的锁使用权分配并不公平（等待最久的不一定能先获得锁的使用权），后者则不会有这个问题。
