# Rust-With-Linked-Lists
阅读Learn Rust by writing Entirely Too Many Linked Lists 后学习写链表

原书中一共实现了6中链表：

- A Bad Stack：仅仅存储i32类型的单向链表，对外提供栈的接口
- An Ok Stack：泛型单向链表，对外提供栈的接口，使用Box
- A Persistent Stack：泛型可持久化单向链表，对外提供栈的接口，使用Rc
- A Bad Safe Deque：safe但功能不全的双向链表，没有实现Iter和IterMut两个迭代器，对外提供双端队列的接口，使用Rc，RefCell
- An Unsafe Queue：*unsafe* 的泛型单向链表，对外提供队列的接口，使用unsafe和Box
- Silly1：基于An Ok Stack实现的 [zipper](https://en.wikipedia.org/wiki/Zipper_(data_structure))

An Ok Unsafe Deque章节原书作者还没写，自己用unsafe实现了一遍这是一个用unsafe实现的双端队列，支持快速在头尾插入和删除元素，补充了A Bad Safe Deque章节中没能实现的Iter和IterMut两个迭代器。
