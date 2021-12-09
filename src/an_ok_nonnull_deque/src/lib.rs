mod iterator;

use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

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

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            next: None,
            prev: None,
            elem,
        }
    }
}

impl<T> List<T> {
    #[inline]
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;
            //Box::leak 这是node节点没有被析构的关键
            // let ptr = Box::into_raw(node);
            // let node = NonNull::new(ptr);
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
            let node = Box::from_raw(node.as_ptr()); //释放节点内存，不然会内存泄漏
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
