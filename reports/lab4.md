# OS Lab4 实验报告

<h4>王哲凡 计93 2019011200</h4>

## 实现功能

- 类似合并 `ch5` 分支，其中 `spawn()` 接口需要添加对应的 `fd_table`。
- 通过在 `DiskInode` 中记录 `nlink` 数量来维护硬链接，并支持 `sys_fstat` 调用。
- 在 `unlinkat` 中，对 `nlink` 降到 $0$ 的类比 `clear()` 和 `dealloc_data()` 来回收数据。

## 问答题

### 第 1 题

root inode 是所有结点的根，也即 `/` 根目录。

在 easy-fs 中，所有的文件都直接位于 `/` 下。所有对于文件的查找、创建、链接、取消链接、列出（`ls`）都需要通过 `ROOT_INODE` 进行，而这又是通过在 `root_inode`（`DiskInode` 类型）中记录各个目录项 `DirEntry` 完成的。

如果 root inode 中的内容损坏，可能导致文件目录项出错，进而出现文件查找不到、文件查找错误、文件创建失败等相关问题。

## 建议

对于 `sys_fstat` 调用的返回值没有明确给出，需要根据测例判断。
