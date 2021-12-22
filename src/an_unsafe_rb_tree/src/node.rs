use std::cmp::{max, Ordering};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ptr::NonNull;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    Red,
    Black,
}

pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub left: Option<NonNull<Node<K, V>>>,
    pub right: Option<NonNull<Node<K, V>>>,
    pub parent: Option<NonNull<Node<K, V>>>,
    pub color: Color,
}

impl<K, V> Node<K, V> {
    fn new_node(key: K, value: V, color: Color, nil: NonNull<Node<K, V>>) -> Self {
        Node {
            key,
            value,
            left: Some(nil), //使用哑节点Nil代替None
            right: Some(nil),
            parent: Some(nil),
            color,
        }
    }

    pub fn new(key: K, value: V, nil: NonNull<Node<K, V>>) -> NonNull<Node<K, V>> {
        let box_node = Box::new(Self::new_node(key, value, Color::Red, nil));
        NonNull::from(Box::leak(box_node))
    }
}

/// 辅助结构体，封装了节点指针和哑节点指针，方便直接对Option<NonNull<Node<K, V>>>操作
pub struct NodeQuery<K, V> {
    node: Option<NonNull<Node<K, V>>>,
    nil: NonNull<Node<K, V>>,
}

impl<K, V> Clone for NodeQuery<K, V> {
    fn clone(&self) -> Self {
        NodeQuery {
            node: self.node,
            nil: self.nil,
        }
    }
}

impl<K: PartialOrd + Clone, V> PartialEq for NodeQuery<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nil() || other.is_nil() {
            return false;
        }
        self.get_key().unwrap() == other.get_key().unwrap()
    }
}

impl<K: PartialOrd + Clone, V> PartialOrd for NodeQuery<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nil() || other.is_nil() {
            return None;
        }
        self.get_key()
            .unwrap()
            .partial_cmp(other.get_key().unwrap())
    }
}

impl<K: PartialOrd + Clone, V> PartialEq<K> for NodeQuery<K, V> {
    fn eq(&self, other: &K) -> bool {
        if self.is_nil() {
            return false;
        }
        self.get_key().unwrap() == other
    }
}

impl<K: PartialOrd + Clone, V> PartialOrd<K> for NodeQuery<K, V> {
    fn partial_cmp(&self, other: &K) -> Option<Ordering> {
        if self.is_nil() {
            return None;
        }
        self.get_key().unwrap().partial_cmp(other)
    }
}

impl<'a, K: Clone + PartialOrd + 'a, V: 'a> NodeQuery<K, V> {
    pub fn new(node: Option<NonNull<Node<K, V>>>, nil: NonNull<Node<K, V>>) -> Self {
        NodeQuery { node, nil }
    }

    /// 返回节点内部的指针
    pub fn inner(&self) -> Option<NonNull<Node<K, V>>> {
        self.node
    }

    /// 返回节点的键
    pub fn get_key(&self) -> Option<&'a K> {
        if self.is_nil() {
            return None;
        }
        self.node.map(|node| unsafe { &node.as_ref().key })
    }

    /// 返回节点的值
    pub fn get_value(&self) -> Option<&'a V> {
        if self.is_nil() {
            return None;
        }
        self.node.map(|node| unsafe { &node.as_ref().value })
    }

    /// 返回节点的键值对
    pub fn get_kv(&self) -> Option<(&'a K, &'a V)> {
        if self.is_nil() {
            return None;
        }
        self.node
            .map(|node| unsafe { (&node.as_ref().key, &node.as_ref().value) })
    }

    /// 返回节点的左子树
    pub fn left(&self) -> Self {
        let left = self.inner().and_then(|node| unsafe { node.as_ref().left });
        Self::new(left, self.nil)
    }

    /// 返回节点的右子树
    pub fn right(&self) -> Self {
        let right = self.inner().and_then(|node| unsafe { node.as_ref().right });
        Self::new(right, self.nil)
    }

    /// 返回节点的双亲
    pub fn parent(&self) -> Self {
        let parent = self
            .inner()
            .and_then(|node| unsafe { node.as_ref().parent });
        Self::new(parent, self.nil)
    }

    /// 返回节点的叔叔
    pub fn uncle(&self) -> Self {
        if self.parent().is_left_child() {
            self.grandparent().right()
        } else {
            self.grandparent().left()
        }
    }

    /// 返回节点的兄弟
    pub fn brother(&self) -> Self {
        if self.is_left_child() {
            self.parent().right()
        } else {
            self.parent().left()
        }
    }

    /// 返回节点的祖父
    pub fn grandparent(&self) -> Self {
        self.parent().parent()
    }

    /// 设置节点的键值对
    pub fn set_entry(&mut self, key: K, value: V) {
        if let Some(mut node) = self.inner() {
            unsafe {
                node.as_mut().key = key;
                node.as_mut().value = value;
            }
        }
    }

    /// 设置节点的颜色
    pub fn set_color(&mut self, color: Color) {
        if let Some(mut node) = self.inner() {
            unsafe { node.as_mut().color = color }
        }
    }

    /// 设置节点的左子树
    pub fn set_left(&mut self, left_node: Option<NonNull<Node<K, V>>>) {
        if let Some(mut node) = self.inner() {
            unsafe {
                node.as_mut().left = left_node;
            }
            if let Some(mut node) = left_node {
                unsafe {
                    node.as_mut().parent = self.inner();
                }
            }
        }
    }

    /// 设置节点的右子树
    pub fn set_right(&mut self, right_node: Option<NonNull<Node<K, V>>>) {
        if let Some(mut node) = self.inner() {
            unsafe {
                node.as_mut().right = right_node;
            }
            if let Some(mut node) = right_node {
                unsafe {
                    node.as_mut().parent = self.inner();
                }
            }
        }
    }

    /// 设置节点的双亲
    pub fn set_parent(&mut self, parent_node: Option<NonNull<Node<K, V>>>) {
        if let Some(mut node) = self.inner() {
            unsafe {
                node.as_mut().parent = parent_node;
            }
        }
    }

    /// 返回节点的颜色
    pub fn color(&self) -> Option<Color> {
        self.inner().map(|node| unsafe { node.as_ref().color })
    }

    /// 判断节点是否为Some
    pub fn is_some(&self) -> bool {
        self.inner().is_some()
    }

    /// 判断节点是否为None
    pub fn is_none(&self) -> bool {
        self.inner().is_none()
    }

    /// 判断节点是否等于哑节点
    pub fn is_nil(&self) -> bool {
        self.is_none() || self.inner() == Some(self.nil)
    }

    /// 判断节点是否为红色
    pub fn is_red(&self) -> bool {
        self.color() == Some(Color::Red)
    }

    /// 判断节点是否为黑色
    pub fn is_black(&self) -> bool {
        self.color() == Some(Color::Black)
    }

    /// 判断节点是否为父亲的左孩子
    pub fn is_left_child(&self) -> bool {
        self.is_some() && self.parent().left().inner() == self.inner()
    }

    /// 判断节点是否为父亲的右孩子
    pub fn is_right_child(&self) -> bool {
        self.is_some() && self.parent().right().inner() == self.inner()
    }

    /// 返回以节点为根的树中的最小节点
    pub fn minimum(&self) -> Self {
        if self.left().is_nil() {
            return Self::new(self.inner(), self.nil);
        }
        let mut cur = self.left();
        while !cur.left().is_nil() {
            cur = cur.left();
        }
        Self::new(cur.inner(), cur.nil)
    }

    /// 返回以节点为根的树中的最大节点
    pub fn maximum(&self) -> Self {
        if self.right().is_nil() {
            return Self::new(self.inner(), self.nil);
        }
        let mut cur = self.right();
        while !cur.right().is_nil() {
            cur = cur.right();
        }
        Self::new(cur.inner(), cur.nil)
    }

    /// 前序遍历
    pub fn pre_order(&self, buf: &mut Vec<K>) {
        if !self.is_nil() {
            buf.push(self.get_key().unwrap().clone());
            self.left().pre_order(buf);
            self.right().pre_order(buf);
        }
    }

    /// 中序遍历
    pub fn in_order(&self, buf: &mut Vec<K>) {
        if !self.is_nil() {
            self.left().in_order(buf);
            buf.push(self.get_key().unwrap().clone());
            self.right().in_order(buf);
        }
    }

    /// 后序遍历
    pub fn post_order(&self, buf: &mut Vec<K>) {
        if !self.is_nil() {
            self.left().post_order(buf);
            self.right().post_order(buf);
            buf.push(self.get_key().unwrap().clone());
        }
    }

    /// 层序遍历
    pub fn level_order(&self, buf: &mut Vec<K>) {
        let mut queue = VecDeque::new();
        if !self.is_nil() {
            queue.push_back(self.clone());
        }
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                buf.push(node.get_key().unwrap().clone());
                if !node.left().is_nil() {
                    queue.push_back(node.left());
                }
                if !node.right().is_nil() {
                    queue.push_back(node.right());
                }
            }
        }
    }

    /// 返回以该节点为根的树高
    pub fn height(&self) -> usize {
        if self.is_nil() {
            return 0;
        }
        let left_height = self.left().height();
        let right_height = self.right().height();
        max(left_height, right_height) + 1
    }
}

impl<K: Clone + PartialOrd + ToString, V: ToString> ToString for NodeQuery<K, V> {
    fn to_string(&self) -> String {
        if self.is_nil() {
            "Ø".to_string()
        } else {
            format!(
                "[K: {}, V: {}, C: {:?} L: {}, R: {}]",
                self.get_key().unwrap().to_string(),
                self.get_value().unwrap().to_string(),
                self.color().unwrap(),
                self.left().to_string(),
                self.right().to_string(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Color, Node, NodeQuery};
    use std::ptr::NonNull;

    #[test]
    fn basic() {
        /*
                    2(b)
                   / \
                 1(b) 3(b)
                       \
                        4(r)
        */
        let nil_node = Box::new(Node {
            key: 0,
            value: 0,
            left: None,
            right: None,
            parent: None,
            color: Color::Black,
        });
        let nil = NonNull::from(Box::leak(nil_node));
        let root_node = Node::new(2, 2, nil);
        let left_node = Node::new(1, 1, nil);
        let mut right_node = Node::new(3, 3, nil);
        let right_right_node = Node::new(4, 4, nil);
        let mut root_query = NodeQuery::new(Some(root_node), nil);
        let mut left_query = NodeQuery::new(Some(left_node), nil);
        let mut right_query = NodeQuery::new(Some(right_node), nil);
        let mut rr_query = NodeQuery::new(Some(right_right_node), nil);
        root_query.set_left(Some(left_node));
        root_query.set_right(Some(right_node));
        rr_query.set_parent(Some(right_node));
        unsafe {
            right_node.as_mut().right = Some(right_right_node);
        }
        root_query.set_color(Color::Black);
        left_query.set_color(Color::Black);
        right_query.set_color(Color::Black);
        // 设置完成
        assert!(!root_query.is_red());
        assert!(root_query.is_some());
        assert!(left_query.is_left_child());
        assert!(right_query.is_right_child());
        assert!(!root_query.is_left_child());
        assert!(!root_query.is_right_child());
        assert!(!left_query.grandparent().is_left_child());

        assert_eq!(root_query.left().node, Some(left_node));
        assert_eq!(root_query.right().node, Some(right_node));
        assert_eq!(left_query.left().node, Some(nil));
        assert_eq!(left_query.right().node, Some(nil));

        assert_eq!(root_query.left().color(), Some(Color::Black));
        assert_eq!(root_query.right().color(), Some(Color::Black));
        assert_eq!(left_query.left().color(), Some(Color::Black));

        assert_eq!(root_query.get_key(), Some(&2));
        assert_eq!(root_query.left().get_key(), Some(&1));
        assert_eq!(root_query.right().get_key(), Some(&3));
        assert_eq!(left_query.left().get_key(), None);

        assert_eq!(rr_query.grandparent().node, Some(root_node));
        assert_eq!(left_query.grandparent().node, Some(nil));
        assert_eq!(root_query.grandparent().node, None);

        assert_eq!(rr_query.uncle().node, Some(left_node));
        assert_eq!(left_query.uncle().node, None);
        assert_eq!(right_query.uncle().node, None);
        assert_eq!(root_query.uncle().node, None);

        assert_eq!(root_query.brother().node, None);
        assert_eq!(left_query.brother().node, right_query.node);
        assert_eq!(right_query.brother().node, left_query.node);
        assert_eq!(rr_query.brother().node, Some(nil));

        assert_eq!(left_query.parent().node, Some(root_node));
        assert_eq!(right_query.parent().node, Some(root_node));
        assert_eq!(root_query.parent().node, Some(nil));

        rr_query.set_entry(5, 5);
        assert_eq!(right_query.right().get_key(), Some(&5));

        assert_eq!(root_query.minimum().node, Some(left_node));
        assert_eq!(left_query.minimum().node, Some(left_node));
        assert_eq!(root_query.maximum().node, Some(right_right_node));
        assert_eq!(right_query.maximum().node, Some(right_right_node));
        assert_eq!(rr_query.maximum().node, Some(right_right_node));
        assert_eq!(rr_query.minimum().node, Some(right_right_node));

        assert_eq!(root_query.to_string(), String::from("[K: 2, V: 2, C: Black L: [K: 1, V: 1, C: Black L: Ø, R: Ø], R: [K: 3, V: 3, C: Black L: Ø, R: [K: 5, V: 5, C: Red L: Ø, R: Ø]]]"));

        assert_eq!(root_query.height(), 3);
        assert_eq!(left_query.height(), 1);
        assert_eq!(right_query.height(), 2);
        assert_eq!(rr_query.height(), 1);
    }

    #[test]
    fn partion_cmp() {
        let nil_node = Box::new(Node {
            key: 0,
            value: 0,
            left: None,
            right: None,
            parent: None,
            color: Color::Black,
        });
        let nil = NonNull::from(Box::leak(nil_node));
        let big_node = Node::new(20, 2, nil);
        let middle_node = Node::new(10, 2, nil);
        let small_node = Node::new(1, 1, nil);
        let eq_small_node = Node::new(1, 1, nil);
        let big_query = NodeQuery::new(Some(big_node), nil);
        let middle_query = NodeQuery::new(Some(middle_node), nil);
        let eq_small_query = NodeQuery::new(Some(eq_small_node), nil);
        let small_query = NodeQuery::new(Some(small_node), nil);
        let none_query = NodeQuery::new(None, nil);
        assert!(big_query > middle_query);
        assert!(middle_query > small_query);
        assert!(small_query < middle_query);
        assert!(small_query == eq_small_query);
        assert_eq!(none_query == small_query, false);
        assert_eq!(none_query > small_query, false);
        assert_eq!(none_query < small_query, false);

        assert!(big_query > 10);
        assert!(middle_query == 10);
        assert!(small_query < 10);
        assert_eq!(none_query == 10, false);
        assert_eq!(none_query > 10, false);
        assert_eq!(none_query < 10, false);
    }
}
