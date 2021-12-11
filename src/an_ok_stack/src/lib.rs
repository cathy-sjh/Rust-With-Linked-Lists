
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => {
                None
            }
            Some(node) => {
                Some(&node.elem)
            }
        }
        // self.head.as_ref().map(|node|{
        //     &node.elem
        // })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match &mut self.head {
            None => {
                None
            }
            Some(node) => {
                Some(&mut node.elem)
            }
        }

        // self.head.as_mut().map(|node| {
        //     &mut node.elem
        // })
    }

    pub fn is_empty(&self) -> bool{
        self.head.is_none()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut box_node) = cur_link {
            cur_link = box_node.next.take();
        }
    }
}

//实现IntoIter迭代器
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

//实现Iter迭代器
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node)
            //next: self.head.as_deref()
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            //self.next = node.next.as_ref().map(|node|&**node);
            //self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

//实现IterMut迭代器
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert!(list.is_empty());

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        assert!(!list.is_empty());

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push("jack".to_string());
        list.push("marry".to_string());
        list.push("hello".to_string());
        assert_eq!(list.peek(), Some(&String::from("hello")));
        assert_eq!(list.peek_mut(), Some(&mut String::from("hello")));

        list.peek_mut().map(|value| {
            value.push_str(" world");
        });

        assert_eq!(list.peek(), Some(&String::from("hello world")));
        assert_eq!(list.pop(), Some(String::from("hello world")));

        // let mut list = List::new();
        // assert_eq!(list.peek(), None);
        // assert_eq!(list.peek_mut(), None);
        // list.push(1); list.push(2); list.push(3);
        //
        // assert_eq!(list.peek(), Some(&3));
        // assert_eq!(list.peek_mut(), Some(&mut 3));

        // // 这里使用了模式匹配，只是将结果的elem复制到了value中,所以并没有改变链表中的值
        // list.peek_mut().map(|&mut mut value| {
        //     value = 13;
        //     println!("value = {}", value);
        // });
        //
        // assert_eq!(list.peek(), Some(&3));
        // assert_eq!(list.pop(), Some(3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
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

        iter.next().map(|elem| {
            *elem = 7
        });

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 5));
        assert_eq!(iter.next(), Some(&mut 6));
        assert_eq!(iter.next(), None);
    }
}
