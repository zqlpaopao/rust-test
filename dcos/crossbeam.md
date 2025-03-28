# 1、crossbeam

Crossbeam是stjepang大神在做smol之前主要贡献的著名Rust的并发库。相信很多人都用过其中的mpmc channel。其实整个Crossbeam对无锁并发有很多支持，极大的提高了并发的性能，是生产代码依赖的不二之选。在这里开这么一个小坑，一半是被大佬们宏大的FFI系列所鼓舞，一半是也想借这个机会好好和大家一起探索一下除了锁之外很多美妙的并发和他们的原理。当然也顺便膜拜一下stjepang大大的杰作。

系列大纲（基本按subcrate组织）：

1. 有锁并发、无锁并发和crossbeam简介
2. crossbeam-epoch：基于epoch的无锁垃圾收集，以及reiber_stack的例子
3. crossbeam-deque：work-stealing算法
4. crossbeam-channel：与std中channel的对比，channel的设计哲学
5. crossbeam-queue和crossbeam-utils：并发队列和杂七杂八的实用小工具
6. crossbeam-skiplist：无锁数据结构之Skip lists



# 2 crossbeam-epoch

当一个线程从并发数据结构中删除一个对象时，其他线程可能仍在同时使用指向该对象的指针，因此无法立即销毁该对象。基于 Epoch 的 GC 是一种有效的机制，用于推迟共享对象的销毁，直到不存在指向它们的指针。

如果启用了该功能，则除了全局 GC 之外，此箱中的所有内容都可以在`no_std`环境 中使用。`alloc`

1. 线程池：`crossbeam-epoch` 提供了线程池 API，让您能够轻松地创建和管理线程。线程池可以帮助您充分利用多核 CPU，同时避免创建过多线程带来的开销。
2. 任务分发：`crossbeam-epoch` 提供了简单易用的任务分发功能，使您能够将任务分配给不同的线程。这可以通过 `spawn_on` 函数实现，该函数接受一个线程 ID 和一个异步函数作为参数。
3. 同步：`crossbeam-epoch` 提供了用于同步线程间操作的 API。例如，`EpochSpawn` 类型实现了 `Sync` 和 `Send` 特性，这意味着您可以在不同的线程之间安全地共享数据。
4. 线程安全性：`crossbeam-epoch` 确保其 API 本身是线程安全的，从而降低了潜在的并发问题风险。
5. Epoch 计数器：`crossbeam-epoch` 提供了 epoch 计数器，用于更好地控制线程的生命周期。这对于需要定期回收不再使用的线程的场景特别有用。

下面是一个简单的 `crossbeam-epoch` 示例，展示了如何创建线程池、分发任务并同步线程操作：

```
```

