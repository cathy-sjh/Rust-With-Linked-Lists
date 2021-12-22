# Rust-With-Linked-Lists
阅读Learn Rust by writing Entirely Too Many Linked Lists 后学习写链表

原书本中一共实现了6中链表：

- A Bad Stack：仅仅存储`i32`类型的单向链表，对外提供栈的接口
- An Ok Stack：泛型单向链表，对外提供栈的接口，使用Box
- A Persistent Stack：泛型可持久化单向链表，对外提供栈的接口，使用Rc
- A Bad Safe Deque：safe但功能不全的双向链表，没有实现Iter和IterMut两个迭代器，对外提供双端队列的接口，使用Rc，RefCell
- An Unsafe Queue：*unsafe* 的泛型单向链表，对外提供队列的接口，使用unsafe和Box
- Silly1：基于An Ok Stack实现的 [zipper](https://en.wikipedia.org/wiki/Zipper_(data_structure))



自己添加的练习内容：

- An Ok Unsafe Deque：由于章节原书作者还没写，自己用unsafe实现了一遍，这是一个用unsafe实现的双端队列，支持快速在头尾插入和删除元素，并且实现的IntoIter、Iter和IterMut三个迭代器的正反向遍历。
- An_OK_nonnoll_deque：标准库中的`LinkedList`使用`NonNull`实现的，它实际上是一种特殊的`*mut T`原生指针，特殊之处有两点：协变和非零，具体可以查看手册[NonNull in std::ptr - Rust (rust-lang.org)](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)。因此模仿标准库中的`LinkedList`实现一个unsafe的双端队列。
- An Ok Binary Tree：基本的二叉树，根据前序遍历创建二叉树，实现前序、中序、后序、层序遍历的迭代和非迭代方法，支持树高、节点数、叶子数的查询，支持第K层节点数查询，查找某个值是否在二叉树中，判断二叉树是否为完全二叉树。
- An Ok Bstree：二叉排序树的实现，支持增删改查，实现`iter`迭代器。
- An Ok Avl Tree：二叉平衡树
- An unsafe rb tree：使用`NonNull`实现的红黑树，参考算法导论第十三章
