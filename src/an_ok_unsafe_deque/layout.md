# Layout

```
[] = Stack
() = Heap
<-->双向链表
[head] --> (*null*, elem A, next) <--> (prev, elem B, next) <--> (prev, elem C, *null*) <-- [tail]
```

可以看到，每个节点都有两个箭头指向它，但是每个节点的所有权只能属于一个所有者，因此这种结构实现起来肯定很困难。所以我将`prev`和`tail`指针变成裸指针。

```
[head]-->(*null*, elem A, next) <--> (raw_prev, elem B, next) <--> (raw_prev, elem C, *null*)<--[raw_tail]
```

这样一来，每个节点的所有权就唯一了，看起来可以实现。

我们将双端队列的结构定义为：

```rust
pub struct List<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
    prev: *mut Node<T>,
}
```

给`Node`实现一些有用的接口：

```rust
impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem,
            next: None,
            prev: std::ptr::null_mut(),
        }
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node::new(T::default())
    }
}
```

接下来实现双端队列的常用接口：

```rust
impl<T: Default> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut new_head = Box::new(Node::new(elem));
        let raw_head: *mut Node<T> = &mut *new_head;
        match self.head.take() {
            None => {
                self.tail = raw_head;
            }
            Some(mut node) => {
                node.prev = raw_head;
                new_head.next = Some(node);
            }
        }
        self.head = Some(new_head);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.next {
                None => {
                    self.tail = std::ptr::null_mut();
                    self.head = None;
                }
                Some(mut new_head) => {
                    new_head.prev = std::ptr::null_mut();
                    self.head = Some(new_head);
                }
            }
            old_head.elem
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }


    pub fn push_back(&mut self, elem: T) {
        let mut new_tail = Box::new(Node::new(elem));
        let raw_tail: *mut Node<T> = &mut *new_tail;
        new_tail.prev = self.tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }
        else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if !self.tail.is_null() {
            unsafe {
                let old_tail = self.tail.replace(Node::default());
                if !old_tail.prev.is_null() {
                    (*old_tail.prev).next = None;
                    self.tail = old_tail.prev;
                }
                else {
                    self.head = None;
                    self.tail = std::ptr::null_mut();
                }
                Some(old_tail.elem)
            }
        }
        else {
            None
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        if !self.tail.is_null() {
            unsafe {
                Some(&(*self.tail).elem)
            }
        }
        else {
            None
        }
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        if !self.tail.is_null() {
            unsafe {
                Some(&mut (*self.tail).elem)
            }
        }
        else {
            None
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_node = self.head.take();
        while let Some(mut node) = cur_node {
            cur_node = node.next.take()
        }
        self.tail = std::ptr::null_mut();
    }
}
```

`push_front`插入值x的操作如下：

原来：

```
[head]-->(*null*, elem A, next) <--> (raw_prev, elem B, *null*)<--[raw_tail]
```

插入后：

```
[head]-->(*null*, elem x, next) <--> (raw_prev, elem A, next) <--> (raw_prev, elem B, *null*)<--[raw_tail]
```

`pop_front`删除x的操作如下：

原来：

```
[head]-->(*null*, elem x, next) <--> (raw_prev, elem A, next) <--> (raw_prev, elem B, *null*)<--[raw_tail]
```

插入后：

```
[head]-->(*null*, elem A, next) <--> (raw_prev, elem B, *null*)<--[raw_tail]
```

注意pop操作在删除X节点之后，一定不要忘了将X节点后面的A节点的`prev`指针置为空。

接下来实现三个迭代器，每个迭代器都有正反遍历操作：

```rust
impl<T: Default> List<T> {
	pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
            prev: self.tail,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
            prev: self.tail,
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T: Default> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    prev: *mut Node<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.prev.is_null() {
            let raw_cur = self.prev;
            unsafe {
                self.prev = (*raw_cur).prev;
                Some(&(*raw_cur).elem)
            }
        }
        else {
            None
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
    prev: *mut Node<T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.prev.is_null() {
            let raw_cur = self.prev;
            unsafe {
                self.prev = (*raw_cur).prev;
                Some(&mut (*raw_cur).elem)
            }
        }
        else {
            None
        }
    }
}
```

接下来测试一下：

```rust
#[cfg(test)]
mod tests {
    use crate::List;

    #[test]
    fn front_basics() {
        let mut list = List::new();
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.pop_front(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn back_basics() {
        let mut list = List::new();
        assert_eq!(list.peek_back(), None);
        assert_eq!(list.pop_back(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.peek_back(), Some(&1));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn front_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.peek_back_mut(), Some(&mut 3));
        list.peek_back_mut().map(|value| {
            *value = 11;
        });
        assert_eq!(list.peek_back_mut(), Some(&mut 11));
        assert_eq!(list.peek_front_mut(), Some(&mut 1));

        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.peek_front_mut(), Some(&mut 3));
        list.peek_front_mut().map(|value| {
            *value = 11;
        });
        assert_eq!(list.peek_front_mut(), Some(&mut 11));
        assert_eq!(list.peek_back_mut(), Some(&mut 1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter_mut();
        iter.next().map(|elem| {
            *elem = 4
        });
        iter.next().map(|elem| {
            *elem = 5
        });

        iter.next().map(|elem| {
            *elem = 6
        });
        for elem in list.iter_mut() {
            *elem += 1;
        }
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn iter_mut_rev() {
        let mut list = List::new();
        list.push_back("apple".to_string());
        list.push_back("banana".to_string());
        list.push_back("sky".to_string());
        let mut iter = list.iter_mut();

        iter.next_back().map(|node| {
            node.push_str(" blue");
        });
        iter.next_back().map(|node| {
            node.push_str(" yello");
        });
        iter.next_back().map(|node| {
            node.push_str(" red");
        });

        for name in list.iter_mut().rev() {
            name.push_str(" yes");
        }
        assert_eq!(list.pop_back(), Some(String::from("sky blue yes")));
        assert_eq!(list.pop_back(), Some(String::from("banana yello yes")));
        assert_eq!(list.pop_back(), Some(String::from("apple red yes")));
        assert_eq!(list.pop_back(), None);
    }
}
```

