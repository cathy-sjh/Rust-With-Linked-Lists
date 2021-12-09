# Layout

```
[] = Stack
() = Heap
<-->双向链表
[head] --> (*null*, elem A, next) <--> (prev, elem B, next) <--> (prev, elem C, *null*) <-- [tail]
```

可以看到，每个节点都有两个箭头指向它，但是每个节点的所有权只能属于一个所有者，因此这种结构实现起来肯定很困难。所以标准库中将head、tail、prev和next指针全部用NonNull实现。

##### NonNull指针

它的定义如下，这实际上是一种特殊的`*mut T`原生指针，他的特殊之处有两点：协变和非零

```rust
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}
```

`NonNull`旨在成为Unsafe Rust默认的原生指针，而非`*const T`和`*mut T`，因为这两者几乎是等价的，可以相互转换，但是不能从`*const T` 直接得到`&mut T`。看看手册怎么说[NonNull in std::ptr - Rust (rust-lang.org)](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)

> *mut T，并且是非零和协变的。
>
> 在使用原生指针构建数据结构时，这通常是正确的使用方法，但由于其附加属性，最终使用起来更危险。 如果您不确定是否应该使用 `NonNull<T>`，只需使用 `*mut T`！ 
>
> 与 `*mut T` 不同，该指针必须始终为非空，即使该指针从未被解引用。 这是为了enum可以使用它来禁止一些值——`Option<NonNull<T>>` 与 `*mut T` 具有相同的大小。但是，如果指针没有被解引用，它仍然可能悬垂。 
>
> 与 `*mut T` 不同，`NonNull<T>` 被选择为对 `T` 进行协变。这使得在构建协变类型时可以使用 `NonNull<T>`，但如果在实际上不应该协变的类型中使用会引入不健全的风险 . （对 `*mut T` 做出了相反的选择，尽管从技术上讲，这种不健全的情况只能由调用不安全的函数引起。） 
>
> 协变适用于大多数安全抽象，例如 Box、Rc、Arc、Vec 和 LinkedList。 之所以如此，是因为它们提供了一个公共 API，该 API 遵循 Rust 的正常共享或者可变规则。 
>
> 如果您的类型不能安全地协变，您必须确保它包含一些附加字段以提供不变性。 通常，此字段将是 PhantomData 类型，如 `PhantomData<Cell<T>>` 或 `PhantomData<&'a mut T>`。 
>
> 注意 `NonNull<T>` 有一个 `&T` 的 `From` 实例。 然而，这并没有改变这样一个事实，即通过（从 a 派生的指针）共享引用进行变异是未定义的行为，除非可变性发生在 `UnsafeCell<T>` 内。 从共享引用创建可变引用也是如此。 在没有 `UnsafeCell<T>` 的情况下使用此 From 实例时，您有责任确保永远不会调用 `as_mut`，并且永远不会将 `as_ptr` 用于突变。 

现在我们的双向队列变成了下面的样子：

```
[NonNull_head]-->(None, elem A, NonNull_next) <--> (NonNull_prev, elem B, NonNull_next) <--> (NonNull_prev, elem C, None)<--[NonNull_tail]
```

但是，这样就有一个问题，裸指针是不拥有所有权的，那谁来保存节点的所有权？这里标准库中使用了`Box::leak(node)`接口，看看标准库怎么说：

> ```rust
> pub fn leak<'a>(b: Box<T, A>) -> &'a mut T
> where
>     A: 'a, 
> ```
>
> 消耗并泄漏 `Box`，返回一个可变引用，`&'a mut T`。请注意，类型 T 必须比选定的生命周期 'a 存活时间更长。 如果该类型只有静态引用，或者根本没有，则可以将其选择为“静态”。
>
> 此函数主要用于在程序生命周期的剩余时间内存在的数据。 删除返回的引用会导致内存泄漏。 如果这是不可接受的，则应首先使用 `Box::from_raw` 函数将引用包装起来，生成一个 Box。 然后可以删除这个 Box，这将正确地销毁 T 并释放分配的内存。
>
> 注意：这是一个关联函数，这意味着您必须将其调用为 `Box::leak(b)` 而不是 `b.leak()`。 这样就不会与内部类型的方法发生冲突。 
>
> 示例：
>
> 简单的使用：
>
> ```rust
> let x = Box::new(41);
> let static_ref: &'static mut usize = Box::leak(x);
> *static_ref += 1;
> assert_eq!(*static_ref, 42);
> ```
>
> 不定长数据
>
> ```rust
> let x = vec![1, 2, 3].into_boxed_slice();
> let static_ref = Box::leak(x);
> static_ref[0] = 4;
> assert_eq!(*static_ref, [4, 2, 3]);
> ```

再来看看`from_raw`接口：

> ```rust
> pub unsafe fn from_raw(raw: *mut T) -> Box<T, Global>
> ```
>
> 从原始指针构造一个Box。 
>
> 调用此函数后，原始指针归生成的 Box 所有。 具体来说，Box 析构函数将调用 T 的析构函数并释放分配的内存。 为了安全起见，必须根据 Box 使用的内存布局分配内存。 
>
> ##### 安全性问题：
>
> 此功能是不安全的，因为使用不当可能会导致内存问题。 例如，如果该函数在同一个原始指针上被调用两次，则可能会发生双重释放。
>
> 内存布局[std::boxed - Rust (rust-lang.org)](https://doc.rust-lang.org/std/boxed/index.html#memory-layout)部分描述了安全条件。 
>
> 示例：
>
> 重新创建一个之前使用 `Box::into_raw` 转换为原始指针的 Box： 
>
> ```rust
> let x = Box::new(5);
> let ptr = Box::into_raw(x);
> let x = unsafe { Box::from_raw(ptr) };
> ```
>
> 使用全局分配器从头开始手动创建一个 Box： 
>
> ```rust
> use std::alloc::{alloc, Layout};
> 
> unsafe {
>     let ptr = alloc(Layout::new::<i32>()) as *mut i32;
>     // In general .write is required to avoid attempting to destruct
>     // the (uninitialized) previous contents of `ptr`, though for this
>     // simple example `*ptr = 5` would have worked as well.
>     ptr.write(5);
>     let x = Box::from_raw(ptr);
> }
> ```

我们将双端队列的结构定义为：

```rust
pub struct List<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    elem: T,
}
```

给`Node`实现一些有用的接口：

```rust
impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            next: None,
            prev: None,
            elem,
        }
    }
}
```

接下来实现双端队列的常用接口：

```rust
impl<T> List<T> {
    #[inline]
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;
            //Box::leak 这是node节点没有被析构的关键的关键
            let node = NonNull::new(Box::leak(node));

            match self.head {
                None => {
                    self.tail = node;
                }
                Some(head) => {
                    (*head.as_ptr()).prev = node;
                }
            }
            self.head = node;
        }
    }

    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            match self.head {
                None => {
                    self.tail = None;
                }
                Some(head) => {
                    (*head.as_ptr()).prev = None;
                }
            }
            node
        })
    }

    #[inline]
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = NonNull::new(Box::leak(node));

            match self.tail {
                None => {
                    self.head = node;
                }
                Some(tail) => {
                    (*tail.as_ptr()).next = node;
                }
            }
            self.tail = node;
        }
    }

    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;
            match self.tail {
                None => {
                    self.head = None;
                }
                Some(tail) => {
                    (*tail.as_ptr()).next = None;
                }
            }
            node
        })
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
            marker: Default::default(),
        }
    }

    pub fn push_front(&mut self, elem: T) {
        self.push_front_node(Box::new(Node::new(elem)))
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.elem)
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().elem) }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().elem) }
    }

    pub fn push_back(&mut self, elem: T) {
        self.push_back_node(Box::new(Node::new(elem)))
    }


    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.elem)
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().elem) }
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().elem) }
    }

    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => {
                mem::swap(self, other)
            }
            Some(mut tail) => {
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }
                }
                self.tail = other.tail.take();
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut List<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                while self.0.pop_front_node().is_some() {}
            }
        }

        while let Some(node) = self.pop_front_node() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
    }
}
```

接下来将List、&List和&mut List转换成迭代器。

```rust
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            marker: Default::default(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            marker: Default::default(),
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
```

接下来实现三个迭代器和相应的trait

```rust
pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| unsafe {
            let node = &*node.as_ptr();
            self.head = node.next;
            &node.elem
        })
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.map(|node| unsafe {
            let node = &*node.as_ptr();
            self.tail = node.prev;
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| unsafe {
            let node = &mut *node.as_ptr();
            self.head = node.next;
            &mut node.elem
        })
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.map(|node| unsafe {
            let node = &mut *node.as_ptr();
            self.tail = node.prev;
            &mut node.elem
        })
    }
}
```
