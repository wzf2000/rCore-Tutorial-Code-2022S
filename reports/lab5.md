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
- 锁/信号量/条件变量的等待队列中，不需要单独回收，因为本框架在锁/信号量/条件变量回收时，等待队列也会相应回收；
- 计时条件变量（`TimerCondVar`）中记录任务线程，需要回收，因为可能存在其他线程此时还处于系统调用 `sleep` 中；
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

这种区别导致，如果前者 `lock` 实现不变，对于等待队列不为空的情况，实际上已经将锁的所有权交给了等待时间最久的线程，但锁还是处于没锁的状态。

如果后续这个线程释放锁，就会导致 `assert!(mutex_inner.locked);` 报错；如果后续线程不释放锁，剩余的在等待队列中的线程就永远不会被唤醒，始终处于等待状态。

后者是框架的实现，不会有这些问题。

特别地，如果前者的 `lock` 实现为，在线程发现锁不空闲时，持续等待到锁不空时，锁上并且继续执行，那么可能导致，此时在 `ready_queue` 中的线程还未执行到请求 `lock`，但在刚加入队列的等待时间最久的线程之前被调度到，且此时会锁成功并继续执行。

这导致锁的使用权分配不再满足原来的先到先得，公平性遭到了破坏。
