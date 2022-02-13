
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: std::ptr::null_mut(),
            }));
            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            }
            else {
                self.head = new_tail;
            }
            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            }
            else {
                let node = Box::from_raw(self.head);
                self.head = node.next;
                if self.head.is_null() {
                    self.tail = std::ptr::null_mut();
                }
                Some(node.elem)
            }

        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| {
                &node.elem
            })
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| {
                &mut node.elem
            })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod test {
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
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);

        assert!(list.is_empty());
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push("jack".to_string());
        list.push("marry".to_string());
        list.push("hello".to_string());
        assert_eq!(list.peek(), Some(&String::from("jack")));
        assert_eq!(list.peek_mut(), Some(&mut String::from("jack")));

        list.peek_mut().map(|value| {
            value.push_str(" hello");
        });

        assert_eq!(list.peek(), Some(&String::from("jack hello")));
        assert_eq!(list.pop(), Some(String::from("jack hello")));

        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek_mut(), Some(&mut 1));
        list.peek_mut().map(|value| {
            *value = 11;
        });
        assert_eq!(list.peek_mut(), Some(&mut 11));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));

        assert_eq!(list.peek(), Some(&1));
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

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}