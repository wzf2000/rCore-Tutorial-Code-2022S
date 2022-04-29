# OS Lab3 实验报告

<h4>王哲凡 计93 2019011200</h4>

## 实现功能

- 通过 `git cherry-pick` 合并 `ch4` 分支。
- 对于 `sys_spawn` 系统调用，新建接口 `spawn()`，手动构建 `task_control_block` 并配置好父子进程信息。
- 通过添加 `priority` 和 `stride` 字段，并修改 `fetch()` 为扫描查询最小值，完成 stride 调度。

## 问答题

### 第 1 题

#### 第 (1) 问

> stride 算法原理非常简单，但是有一个比较大的问题。例如两个 `pass = 10` 的进程，使用 8bit 无符号整型储存 stride， `p1.stride = 255`, `p2.stride = 250`，在 `p2` 执行一个时间片后，理论上下一次应该 `p1` 执行。

> 实际情况是轮到 `p1` 执行吗？为什么？

**答**：不是，由于使用了 8bit 无符号整型储存，经过了 `p2.stride += pass` 后，由于溢出，`p2.stride = 10`，反而 `p2.stride < p1.stride`，使得 `p2` 继续执行（并且会在之后多个连续时间片中持续占用内核）。

#### 第 (2) 问

> 我们之前要求进程优先级 `>= 2` 其实就是为了解决这个问题。可以证明， 在不考虑溢出的情况下 , 在进程优先级全部 `>= 2` 的情况下，如果严格按照算法执行，那么 `STRIDE_MAX – STRIDE_MIN <= BigStride / 2`。

> 为什么？尝试简单说明（不要求严格证明）。

**答**：考虑某一时间点时，满足所有的进程优先级 `STRIDE_MAX – STRIDE_MIN <= BigStride / 2` 也即 `STRIDE_MAX <= STRIDE_MIN + BigStride / 2`。

由于任意进程的优先级均满足 `>= 2`，因此对于最小 stride 的进程 `pmin`，`pmin.pass <= BigStride / 2`，故在 stride 更新完成后，`pmin.stride = STRIDE_MIN + pmin.pass <= STRIDE_MIN + BigStride / 2`。

更新后，新的 stride 最小值 `NEW_STRIDE_MIN >= STRIDE_MIN`，新的 stride 最大值 `NEW_STRIDE_MAX = max(STRIDE_MAX, STRIDE_MIN + pmin.pass) <= max(STRIDE_MIN + BigStride / 2, STRIDE_MIN + BigStride / 2) = STRIDE_MIN + BigStride / 2`。

因此 `NEW_STRIDE_MAX - NEW_STRIDE_MIN <= STRIDE_MIN + BigStride / 2 - STRIDE_MIN = BigStride / 2`，由数学归纳法及初始条件满足即可得知。

#### 第 (3) 问

> 已知以上结论，考虑溢出的情况下，可以为 `Stride` 设计特别的比较器，让 `BinaryHeap<Stride>` 的 `pop` 方法能返回真正最小的 Stride。补全下列代码中的 `partial_cmp` 函数，假设两个 `Stride` 永远不会相等。

> TIPS: 使用 8 bits 存储 stride, `BigStride = 255`, 则: `(125 < 255) == false, (129 < 255) == true`.

**答**：下面代码用 8 bits 存储 stride，即 `Stride` 的值位于 $0 \sim 255$，并且假设 `BigStride = 255`。

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let BigStride = 255;
        if self.0 < other.0 && other.0 - self.0 <= BigStride / 2 || self.0 > other.0 && self.0 - other.0 > BigStride / 2 {
            Some(Ordering::Greater)
        }
        else {
            Some(Ordering::Less)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

## 建议

向前兼容需要花费的精力不小于完成本实验，我建议修改实验框架降低兼容难度。
