use std::collections::VecDeque;
use crate::node::Node;
use std::fmt::Debug;
use crate::iterator::TraverseIter;

pub struct BTree<T> {
    root: Node<T>,
    degree: usize,
}

impl<T: PartialOrd + Clone + Debug> BTree<T> {
    /// 构建一棵空的B树,设置B树的度t = 2,
    /// B树的度t必须满足t ≥ 2,
    /// 除了根节点以外的每个节点必须至少有t - 1个关键字，
    /// 除了根节点以外的每个节点必须至少有t个孩子，
    /// 每个节点最多包含2t-1个关键字，每个内部节点最多有2t个孩子
    /// # Examples
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree: BTree<i32> = BTree::new(2);
    /// ```
    pub fn new(degree: usize) -> Self {
        let new_node = Node::new(degree, None, None);
        BTree {
            root: new_node,
            degree,
        }
    }

    /// 判断当前B树是否为空
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree: BTree<i32> = BTree::new(2);
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.children_len() == 0 && self.root.key_len() == 0
    }

    /// 向B树中插入关键字,支持重复插入
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// tree.insert(1);
    /// assert!(tree.contain(&1));
    /// tree.insert(2);
    /// assert!(tree.contain(&2));
    /// ```
    pub fn insert(&mut self, key: T) {
        if self.root.is_full_keys() {
            let mut new_root = Node::new(self.degree, None, None);
            std::mem::swap(&mut self.root, &mut new_root);
            self.root.insert_child(0, new_root);
            self.root.split_child(0);
        }
        self.root.insert_non_full(key);
    }

    /// 从B树中删除关键字，就算没找到关键字也会调整树的结构
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// tree.insert(1);
    /// tree.delete(1);
    /// assert!(tree.is_empty());
    /// tree.delete(2);
    /// assert!(tree.is_empty());
    /// ```
    pub fn delete(&mut self, key: T) {
        self.root.delete(key);
    }

    /// 查找是否存在关键字
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// tree.insert(1);
    /// assert_eq!(tree.contain(&1), true);
    /// assert_eq!(tree.contain(&2), false);
    /// ```
    pub fn contain(&self, key: &T) -> bool {
        self.root.search(key).is_some()
    }

    /// 返回B树中的最大关键字
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// assert_eq!(tree.find_max(), None);
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// assert_eq!(tree.find_max(), Some(3));
    /// ```
    pub fn find_max(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.root.max_key())
    }

    /// 返回B树中的最小关键字
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// assert_eq!(tree.find_min(), None);
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// assert_eq!(tree.find_min(), Some(1));
    /// ```
    pub fn find_min(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.root.min_key())
    }

    /// 返回B树中的给定关键字的后继，如果给定节点不存在或后继不存在则返回None
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// assert_eq!(tree.successor(1), Some(2));
    /// assert_eq!(tree.successor(3), None);
    /// assert_eq!(tree.successor(0), None);
    /// ```
    pub fn successor(&self, key: T) -> Option<T> {
        if let Some((node, index)) = self.root.search(&key) {
            return if !node.is_leaf() {
                Some(node.get_child(index + 1).min_key())
            } else if index < node.key_len() - 1 {
                Some(node.get_key(index + 1).clone())
            } else {
                let mut current = &self.root;
                let mut succ = None;
                while !current.is_leaf() {
                    let mut i = 0;
                    while i < current.key_len() && key > *current.get_key(i) {
                        i += 1;
                    }
                    if i < current.key_len() && key < *current.get_key(i) {
                        succ = Some(current.get_key(i).clone());
                    }
                    current = current.get_child(i);
                }
                succ
            }
        }
        None
    }

    /// 返回B树中的给定关键字的前驱，如果给定节点不存在或后继不存在则返回None
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// assert_eq!(tree.predecessor(3), Some(2));
    /// assert_eq!(tree.predecessor(1), None);
    /// assert_eq!(tree.predecessor(4), None);
    /// ```
    pub fn predecessor(&self, key: T) -> Option<T> {
        if let Some((node, index)) = self.root.search(&key) {
            return if !node.is_leaf() {
                Some(node.get_child(index).max_key())
            } else if index > 0 {
                Some(node.get_key(index - 1).clone())
            } else {
                let mut current = &self.root;
                let mut pred = None;
                while !current.is_leaf() {
                    let mut i = 0;
                    while i < current.key_len() && key > *current.get_key(i) {
                        pred = Some(current.get_key(i).clone());
                        i += 1;
                    }
                    current = current.get_child(i);
                }
                pred
            }
        }
        None
    }

    /// 中序遍历迭代器
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// tree.insert(3);
    /// tree.insert(2);
    /// tree.insert(1);
    /// tree.insert(4);
    /// let res: Vec<i32> = tree.inorder_iter().collect();
    /// assert_eq!(res, vec![1,2,3,4]);
    /// ```
    pub fn inorder_iter(&self) -> TraverseIter<T>{
        let mut buf = Vec::new();
        self.root.in_order(&mut buf);
        let mut queue = VecDeque::new();
        for x in buf {
            queue.push_back(x);
        }
        TraverseIter::new(queue)
    }

    /// 层序遍历迭代器
    /// # Example
    /// ```
    /// use an_ok_btree::BTree;
    /// let mut tree = BTree::new(2);
    /// tree.insert(3);
    /// tree.insert(2);
    /// tree.insert(1);
    /// tree.insert(4);
    /// let res: Vec<i32> = tree.levelorder_iter().collect();
    /// assert_eq!(res, vec![2,1,3,4]);
    /// ```
    pub fn levelorder_iter(&self) -> TraverseIter<T>{
        let mut buf = Vec::new();
        self.root.level_order(&mut buf);

        let mut queue = VecDeque::new();
        for x in buf {
            queue.push_back(x);
        }
        TraverseIter::new(queue)
    }
}

impl<T: Debug> ToString for BTree<T> {
    fn to_string(&self) -> String {
        format!("{:?}", self.root)
    }
}
