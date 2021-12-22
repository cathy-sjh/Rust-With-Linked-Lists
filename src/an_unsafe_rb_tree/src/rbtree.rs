use crate::iterator::TraverseIter;
use crate::node::{Color, Node, NodeQuery};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// 红黑树是满足以下性质的自平衡二叉搜索树:
///
/// 1.每个节点要么是红色，要么是黑色
///
/// 2.根节点是黑色
///
/// 3.每个叶节点(NIL)是黑色
///
/// 4.如果一个节点是红色，则他的两个子节点都是黑色
///
/// 5.对每个节点，从该节点到其所有后代叶节点的简单路径上，均包含相同数目的黑色节点
///
pub struct RBTree<K, V> {
    root: Option<NonNull<Node<K, V>>>,
    nil: NonNull<Node<K, V>>,
    marker: PhantomData<Box<Node<K, V>>>,
}

impl<K: Default + PartialOrd + Clone, V: Default> RBTree<K, V> {
    /// 构建一棵空的红黑树
    /// # Examples
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree: RBTree<i32, i32> = RBTree::new();
    /// ```
    pub fn new() -> Self {
        // 构建哨兵节点nil，为了方便处理红黑树中的边界条件。color属性为Black，其他属性可以任意
        let nil_node = Box::new(Node {
            key: K::default(),
            value: V::default(),
            left: None,
            right: None,
            parent: None,
            color: Color::Black,
        });
        RBTree {
            root: None,
            nil: NonNull::from(Box::leak(nil_node)),
            marker: Default::default(),
        }
    }

    /// 向红黑树中插入键值对，如果键已经存在，则替换旧值为新值
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get(&1), Some(&'a'));
    /// tree.insert(2, 'b');
    /// assert_eq!(tree.get(&2), Some(&'b'));
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        let mut y_node = Some(self.nil);
        let mut x = NodeQuery::new(self.root, self.nil);

        while !x.is_nil() {
            y_node = x.inner();
            if x > key {
                x = x.left();
            } else if x < key {
                x = x.right();
            } else {
                x.set_entry(key, value);
                return;
            }
        }
        let new_node = Node::new(key, value, self.nil);
        let mut z = NodeQuery::new(Some(new_node), self.nil);
        let mut y = NodeQuery::new(y_node, self.nil);
        z.set_parent(y.inner());
        if y.is_nil() {
            self.root = z.inner();
        } else if z.get_key() < y.get_key() {
            y.set_left(z.inner());
        } else {
            y.set_right(z.inner());
        }
        self.insert_fixup(z.inner());
    }

    /// 从红黑树中删除键值对，如果找不到键值对，则忽略
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(1, 'a');
    /// tree.delete(1);
    /// assert!(tree.is_empty());
    /// tree.delete(2);
    /// assert!(tree.is_empty());
    /// ```
    pub fn delete(&mut self, key: K) {
        let delete_node = self.search(&key);
        if delete_node.is_none() {
            return;
        }
        let z = NodeQuery::new(delete_node, self.nil); //待删除节点
        let mut y = z.clone(); // 用于替换待删除节点
        let mut x; // 待删除节点的右子节点
        let mut y_original_color = y.color().unwrap();
        if z.left().is_nil() {
            x = z.right();
            self.transplant(z.inner(), z.right().inner());
        } else if z.right().is_nil() {
            x = z.left();
            self.transplant(z.inner(), z.left().inner());
        } else {
            y = z.right().minimum();
            y_original_color = y.color().unwrap();
            x = y.right();
            if y.parent().inner() == z.inner() {
                x.set_parent(y.inner());
            } else {
                self.transplant(y.inner(), y.right().inner());
                y.set_right(z.right().inner());
                y.right().set_parent(y.inner());
            }
            self.transplant(z.inner(), y.inner());
            y.set_left(z.left().inner());
            y.left().set_parent(y.inner());
            y.set_color(z.color().unwrap());
        }
        if y_original_color == Color::Black {
            self.delete_fixup(x.inner());
        }
        // 释放删除节点的内存
        if let Some(delete_ptr) = delete_node {
            let _ = unsafe { Box::from_raw(delete_ptr.as_ptr()) };
        }
    }

    /// 判断当前红黑树是否为空
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree: RBTree<i32, i32> = RBTree::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        NodeQuery::new(self.root, self.nil).is_nil()
    }

    /// 根据键查找对应的值，找不到返回None，返回值的不可变借用
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get(&1), Some(&'a'));
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let p = self.search(key);
        NodeQuery::new(p, self.nil).get_value()
    }

    /// 根据键获取相应键值对
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get_pair(&1), Some((&1, &'a')));
    /// ```
    pub fn get_pair(&self, key: &K) -> Option<(&K, &V)> {
        let p = self.search(key);
        NodeQuery::new(p, self.nil).get_kv()
    }

    /// 查找是否存在键值对
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.contains(&1), true);
    /// assert_eq!(tree.contains(&2), false);
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.search(key).is_some()
    }

    /// 返回红黑树中的最小键值对
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.min_pair(), Some((&1, &'a')));
    /// ```
    pub fn min_pair(&self) -> Option<(&K, &V)> {
        NodeQuery::new(self.root, self.nil).minimum().get_kv()
    }

    /// 返回红黑树中的最大键值对
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.max_pair(), Some((&3, &'c')));
    /// ```
    pub fn max_pair(&self) -> Option<(&K, &V)> {
        NodeQuery::new(self.root, self.nil).maximum().get_kv()
    }

    ///返回红黑树中给定节点的后继，如果给定节点不存在或后继不存在则返回None
    /// # Example
    ///```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.successor(&1), Some((&2, &'b')));
    /// assert_eq!(tree.successor(&0), None);
    /// assert_eq!(tree.successor(&3), None);
    /// ```
    pub fn successor(&self, key: &K) -> Option<(&K, &V)> {
        let cur = self.search(key)?;
        let mut x = NodeQuery::new(Some(cur), self.nil);
        if !x.right().is_nil() {
            return x.right().minimum().get_kv();
        }
        let mut y = x.parent();
        while !y.is_nil() && x.is_right_child() {
            x = y.clone();
            y = y.parent();
        }
        if y.is_nil() {
            return None;
        }
        y.get_kv()
    }

    ///返回红黑树中给定节点的前驱，如果给定节点不存在或前驱不存在则返回None
    /// # Example
    ///```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.predecessor(&3), Some((&2, &'b')));
    /// assert_eq!(tree.predecessor(&1), None);
    /// assert_eq!(tree.predecessor(&6), None);
    /// ```
    pub fn predecessor(&self, key: &K) -> Option<(&K, &V)> {
        let cur = self.search(key)?;
        let mut x = NodeQuery::new(Some(cur), self.nil);
        if !x.left().is_nil() {
            return x.left().maximum().get_kv();
        }
        let mut y = x.parent();
        while !y.is_nil() && x.is_left_child() {
            x = y.clone();
            y = y.parent();
        }
        if y.is_nil() {
            return None;
        }
        y.get_kv()
    }

    /// 前序遍历迭代器
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.preorder_iter().collect();
    /// assert_eq!(res, vec![(&2, &'b'), (&1, &'a'), (&3, &'c')]);
    /// ```
    pub fn preorder_iter(&self) -> TraverseIter<K, V> {
        let pre_order = self.prev_order();
        let mut queue = VecDeque::new();
        for key in pre_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    /// 中序遍历迭代器
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.inorder_iter().collect();
    /// assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&3, &'c')]);
    /// ```
    pub fn inorder_iter(&self) -> TraverseIter<K, V> {
        let in_order = self.in_order();
        let mut queue = VecDeque::new();
        for key in in_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    /// 后序遍历迭代器
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.postorder_iter().collect();
    /// assert_eq!(res, vec![(&1, &'a'), (&3, &'c'), (&2, &'b')]);
    /// ```
    pub fn postorder_iter(&self) -> TraverseIter<K, V> {
        let post_order = self.post_order();
        let mut queue = VecDeque::new();
        for key in post_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    /// 层序遍历迭代器
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// let res: Vec<(&i32, &char)> = tree.levelorder_iter().collect();
    /// assert_eq!(res, vec![(&2, &'b'), (&1, &'a'), (&3, &'c')]);
    /// ```
    pub fn levelorder_iter(&self) -> TraverseIter<K, V> {
        let level_order = self.level_order();
        let mut queue = VecDeque::new();
        for key in level_order {
            if let Some(p) = self.get_pair(&key) {
                queue.push_back(p);
            }
        }
        TraverseIter::new(queue)
    }

    /// 测量树高
    /// # Example
    /// ```
    /// use an_unsafe_rb_tree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.tree_height(), 2);
    /// ```
    pub fn tree_height(&self) -> usize {
        NodeQuery::new(self.root, self.nil).height()
    }
}

impl<K: Default + PartialOrd + Clone, V: Default> RBTree<K, V> {
    // 插入后修复红黑树，使之继续保持红黑树性质
    fn insert_fixup(&mut self, node: Option<NonNull<Node<K, V>>>) {
        let mut z = NodeQuery::new(node, self.nil);
        while z.parent().is_red() {
            let mut y = z.uncle();
            if z.parent().is_left_child() {
                if y.is_red() {
                    z.parent().set_color(Color::Black); //case1
                    y.set_color(Color::Black); //case1
                    z.grandparent().set_color(Color::Red); //case1
                    z = z.grandparent(); //case1
                } else {
                    if z.is_right_child() {
                        //case2
                        z = z.parent(); //case2
                        self.left_rotate(z.inner()); //case2
                    }
                    z.parent().set_color(Color::Black); //case3
                    z.grandparent().set_color(Color::Red); //case3
                    self.right_rotate(z.grandparent().inner()); //case3
                }
            } else {
                if y.is_red() {
                    z.parent().set_color(Color::Black);
                    y.set_color(Color::Black);
                    z.grandparent().set_color(Color::Red);
                    z = z.grandparent();
                } else {
                    if z.is_left_child() {
                        z = z.parent();
                        self.right_rotate(z.inner());
                    }
                    z.parent().set_color(Color::Black);
                    z.grandparent().set_color(Color::Red);
                    self.left_rotate(z.grandparent().inner());
                }
            }
        }
        let mut t = NodeQuery::new(self.root, self.nil);
        t.set_color(Color::Black);
    }

    // 删除后修复红黑树，使之继续保持红黑树性质
    fn delete_fixup(&mut self, node: Option<NonNull<Node<K, V>>>) {
        let mut x = NodeQuery::new(node, self.nil);
        while x.inner() != self.root && x.is_black() {
            let mut w = x.brother();
            if x.is_left_child() {
                if w.is_red() {
                    w.set_color(Color::Black); //case1
                    x.parent().set_color(Color::Red); //case1
                    self.left_rotate(x.parent().inner()); //case1
                    w = x.brother(); //case1
                }
                if w.left().is_black() && w.right().is_black() {
                    w.set_color(Color::Red); //case2
                    x = x.parent(); //case2
                } else {
                    if w.right().is_black() {
                        //case3
                        w.left().set_color(Color::Black); //case3
                        w.set_color(Color::Red); //case3
                        self.right_rotate(w.inner()); //case3
                        w = x.brother(); //case3
                    }
                    w.set_color(x.parent().color().unwrap()); //case4
                    x.parent().set_color(Color::Black); //case4
                    w.right().set_color(Color::Black); //case4
                    self.left_rotate(x.parent().inner()); //case4
                    x = NodeQuery::new(self.root, self.nil); //case4
                }
            } else {
                if w.is_red() {
                    w.set_color(Color::Black);
                    x.parent().set_color(Color::Red);
                    self.right_rotate(x.parent().inner());
                    w = x.brother();
                }
                if w.right().is_black() && w.left().is_black() {
                    w.set_color(Color::Red);
                    x = x.parent();
                } else {
                    if w.left().is_black() {
                        w.right().set_color(Color::Black);
                        w.set_color(Color::Red);
                        self.left_rotate(w.inner());
                        w = x.brother();
                    }
                    w.set_color(x.parent().color().unwrap());
                    x.parent().set_color(Color::Black);
                    w.left().set_color(Color::Black);
                    self.right_rotate(x.parent().inner());
                    x = NodeQuery::new(self.root, self.nil);
                }
            }
        }
        x.set_color(Color::Black);
    }

    // 左旋转
    fn left_rotate(&mut self, node: Option<NonNull<Node<K, V>>>) {
        let mut x = NodeQuery::new(node, self.nil);
        let mut y = x.right();
        x.set_right(y.left().inner());
        if !y.left().is_nil() {
            y.left().set_parent(x.inner());
        }
        y.set_parent(x.parent().inner());
        if x.parent().is_nil() {
            self.root = y.inner();
        } else if x.is_left_child() {
            x.parent().set_left(y.inner());
        } else {
            x.parent().set_right(y.inner());
        }
        y.set_left(x.inner());
        x.set_parent(y.inner());
    }

    // 右旋转
    fn right_rotate(&mut self, node: Option<NonNull<Node<K, V>>>) {
        let mut x = NodeQuery::new(node, self.nil);
        let mut y = x.left();
        x.set_left(y.right().inner());
        if !y.right().is_nil() {
            y.right().set_parent(x.inner());
        }
        y.set_parent(x.parent().inner());
        if x.parent().is_nil() {
            self.root = y.inner();
        } else if x.is_left_child() {
            x.parent().set_left(y.inner());
        } else {
            x.parent().set_right(y.inner());
        }
        y.set_right(x.inner());
        x.set_parent(y.inner());
    }

    // 根据键查找节点
    fn search(&self, key: &K) -> Option<NonNull<Node<K, V>>> {
        let mut x = NodeQuery::new(self.root, self.nil);
        while !x.is_nil() {
            if x > *key {
                x = x.left();
            } else if x < *key {
                x = x.right();
            } else {
                break;
            }
        }
        if x.is_nil() {
            return None;
        }
        x.inner()
    }

    //delete 调用的子过程,用src替换dest的位置
    fn transplant(&mut self, dest: Option<NonNull<Node<K, V>>>, src: Option<NonNull<Node<K, V>>>) {
        let u = NodeQuery::new(dest, self.nil);
        let mut v = NodeQuery::new(src, self.nil);
        if u.parent().is_nil() {
            self.root = src;
        } else if u.is_left_child() {
            u.parent().set_left(src);
        } else {
            u.parent().set_right(src);
        }
        v.set_parent(u.parent().inner())
    }

    // 前序遍历
    fn prev_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        let q = NodeQuery::new(self.root, self.nil);
        q.pre_order(&mut buf);
        buf
    }

    // 中序遍历
    fn in_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        let q = NodeQuery::new(self.root, self.nil);
        q.in_order(&mut buf);
        buf
    }

    // 后序遍历
    fn post_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        let q = NodeQuery::new(self.root, self.nil);
        q.post_order(&mut buf);
        buf
    }

    // 层序遍历
    fn level_order(&self) -> Vec<K> {
        let mut buf = Vec::new();
        let q = NodeQuery::new(self.root, self.nil);
        q.level_order(&mut buf);
        buf
    }
}

/// 将红黑树打印成字符串
/// # Example
/// ```
/// use an_unsafe_rb_tree::RBTree;
/// let mut tree = RBTree::new();
/// tree.insert(1, 'a');
/// assert_eq!(tree.to_string(), "[K: 1, V: a, C: Black L: Ø, R: Ø]".to_string());
/// ```
impl<K: Clone + PartialOrd + ToString, V: ToString> ToString for RBTree<K, V> {
    fn to_string(&self) -> String {
        NodeQuery::new(self.root, self.nil).to_string()
    }
}

impl<K: Default + PartialOrd + Clone, V: Default> Default for RBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Drop for RBTree<K, V> {
    fn drop(&mut self) {
        fn visitor<K, V>(node_ptr: Option<NonNull<Node<K, V>>>, nil: NonNull<Node<K, V>>) {
            if let Some(p) = node_ptr {
                if p == nil {
                    return;
                }
                let node = unsafe { Box::from_raw(p.as_ptr()) };
                visitor(node.left, nil);
                visitor(node.right, nil);
            }
        }
        visitor(self.root, self.nil);
        let _nil_node = unsafe { Box::from_raw(self.nil.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use crate::RBTree;

    #[test]
    fn search() {
        let mut tree = RBTree::new();
        tree.insert(1, 'a');
        tree.insert(2, '3');
        tree.insert(3, '3');
        assert!(tree.search(&1).is_some());
        assert!(tree.search(&2).is_some());
        assert!(tree.search(&3).is_some());
        assert!(tree.search(&0).is_none());
        assert!(tree.search(&4).is_none());
    }
}
