use crate::iterator::TraverseIter;
use std::collections::VecDeque;
use crate::bstree_no_recursion::node::{Link, Node};

pub struct BSTree<K, V> {
    root: Link<K, V>,
}

impl<K: PartialOrd + Clone, V> Default for BSTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialOrd + Clone, V> BSTree<K, V> {
    /// 构建一棵空的二叉查找树
    /// # Examples
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree: BSTree<i32, i32> = BSTree::new();
    /// ```
    pub fn new() -> Self {
        BSTree { root: None }
    }

    /// 判断当前树是否为空
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree: BSTree<i32, i32> = BSTree::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// 向树中插入键值对，如果键已经存在，则替换旧值为新值
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get(&1), Some(&'a'));
    /// tree.insert(2, 'b');
    /// assert_eq!(tree.get(&2), Some(&'b'));
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(key, value)));
            return;
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current.take() {
            if key < cur.key {
                if cur.left.is_none() {
                    cur.left = Some(Box::new(Node::new(key, value)));
                    return;
                } else {
                    current = cur.left.as_mut();
                }
            } else if key > cur.key {
                if cur.right.is_none() {
                    cur.right = Some(Box::new(Node::new(key, value)));
                    return;
                } else {
                    current = cur.right.as_mut();
                }
            } else {
                cur.value = value;
                return;
            }
        }
    }

    /// 从树中删除键值对，如果找不到键值对，则忽略
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(1, 'a');
    /// tree.delete(1);
    /// assert!(tree.is_empty());
    /// tree.delete(2);
    /// assert!(tree.is_empty());
    /// ```
    pub fn delete(&mut self, key: K) {
        if let Some(root) = self.root.take() {
            if root.key == key {
                self.root = root.delete_root();
                return;
            }
            else {
                self.root = Some(root);
            }
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current.take() {
            if key < cur.key {
                if let Some(left) = cur.left.take() {
                    if left.key == key {
                        cur.left = left.delete_root();
                    } else {
                        cur.left = Some(left);
                        current = cur.left.as_mut();
                    }
                }
            } else if key > cur.key {
                if let Some(right) = cur.right.take() {
                    if right.key == key {
                        cur.right = right.delete_root();
                    } else {
                        cur.right = Some(right);
                        current = cur.right.as_mut();
                    }
                }
            }
        }
    }

    /// 根据键获取相应键值对
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get_pair(&1), Some((&1, &'a')));
    /// ```
    pub fn get_pair(&self, key: &K) -> Option<(&K, &V)> {
        let mut current = self.root.as_ref();
        while let Some(node) = current.take() {
            if *key < node.key {
                current = node.left.as_ref();
            } else if *key > node.key {
                current = node.right.as_ref();
            } else {
                return Some((&node.key, &node.value));
            }
        }
        None
    }

    /// 根据键查找对应的值，找不到返回None，返回值的不可变借用
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get(&1), Some(&'a'));
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_pair(key).map(|node|node.1)
    }

    /// 据键查找对应的值，找不到返回默认值
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.get_or(&1, &'z'), &'a');
    /// assert_eq!(tree.get_or(&2, &'z'), &'z');
    /// ```
    pub fn get_or<'a>(&'a self, key: &K, default: &'a V) -> &'a V {
        self.get(key).map_or(default, |data| data)
    }

    /// 查找是否存在键值对
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.contains(&1), true);
    /// assert_eq!(tree.contains(&2), false);
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// 返回树中的最大键值对
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.max_pair(), Some((&3, &'c')));
    /// ```
    pub fn max_pair(&self) -> Option<(&K, &V)> {
        let mut current = self.root.as_ref();
        while let Some(cur) = current.take() {
            if cur.right.is_none() {
                return Some((&cur.key, &cur.value));
            }
            else {
                current = cur.right.as_ref();
            }
        }
        None
    }

    /// 返回树中的最小键值对
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.min_pair(), Some((&1, &'a')));
    /// ```
    pub fn min_pair(&self) -> Option<(&K, &V)> {
        let mut current = self.root.as_ref();
        while let Some(cur) = current.take() {
            if cur.left.is_none() {
                return Some((&cur.key, &cur.value));
            }
            else {
                current = cur.left.as_ref();
            }
        }
        None
    }

    ///返回第一个大于key的键值对
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.successor(&1), Some((&2, &'b')));
    /// assert_eq!(tree.successor(&0), Some((&1, &'a')));
    /// assert_eq!(tree.successor(&3), None);
    /// ```
    pub fn successor(&self, key: &K) -> Option<(&K, &V)> {
        let mut current = self.root.as_ref();
        let mut successor = None;
        while let Some(node) = current {
            if node.key > *key {
                successor = current;
                current = node.left.as_ref();
            } else {
                current = node.right.as_ref();
            }
        }
        successor.map(|node| (&node.key, &node.value))
    }

    ///返回第一个小于key的键值对
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// assert_eq!(tree.predecessor(&3), Some((&2, &'b')));
    /// assert_eq!(tree.predecessor(&5), Some((&3, &'c')));
    /// assert_eq!(tree.predecessor(&1), None);
    /// ```
    pub fn predecessor(&self, key: &K) -> Option<(&K, &V)> {
        let mut current = self.root.as_ref();
        let mut predecessor = None;
        while let Some(node) = current {
            if node.key < *key {
                predecessor = current;
                current = node.right.as_ref();
            } else {
                current = node.left.as_ref();
            }
        }
        predecessor.map(|node| (&node.key, &node.value))
    }

    ///删除以val为根节点的树枝
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// tree.insert(4, 'd');
    /// tree.delete_tree(2);
    /// assert!(!tree.contains(&2));
    /// assert!(!tree.contains(&1));
    /// tree.delete_tree(3);
    /// assert!(tree.is_empty());
    /// ```
    pub fn delete_tree(&mut self, key: K) {
        if let Some(root) = self.root.take() {
            if root.key == key {
                self.root.take();
                return;
            }
            else {
                self.root = Some(root);
            }
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current.take() {
            if key < cur.key {
                if let Some(left) = cur.left.as_mut() {
                    if left.key == key {
                        cur.left.take();
                    } else {
                        current = cur.left.as_mut();
                    }
                }
            } else if key > cur.key {
                if let Some(right) = cur.right.as_mut() {
                    if right.key == key {
                        cur.right.take();
                    } else {
                        current = cur.right.as_mut();
                    }
                }
            }
        }
    }

    ///删除以val为根节点的树枝, 并返回切掉的树枝
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(3, 'c');
    /// tree.insert(2, 'b');
    /// tree.insert(1, 'a');
    /// tree.insert(4, 'd');
    /// let rm_tree = tree.remove_tree(2);
    /// assert!(rm_tree.contains(&2));
    /// assert!(rm_tree.contains(&1));
    /// let rm_tree = tree.remove_tree(12);
    /// assert!(rm_tree.is_empty());
    /// ```
    pub fn remove_tree(&mut self, key: K) -> Self {
        if let Some(root) = self.root.take() {
            if root.key == key {
                return Self {root: Some(root)}
            }
            else {
                self.root = Some(root);
            }
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current.take() {
            if key < cur.key {
                if let Some(left) = cur.left.take() {
                    if left.key == key {
                        return Self {
                            root: Some(left),
                        };
                    } else {
                        cur.left = Some(left);
                        current = cur.left.as_mut();
                    }
                }
            } else if key > cur.key {
                if let Some(right) = cur.right.take() {
                    if right.key == key {
                        return Self {
                            root: Some(right),
                        };
                    } else {
                        cur.right = Some(right);
                        current = cur.right.as_mut();
                    }
                }
            }
        }
        Self::new()
    }

    //前序遍历非迭代法
    fn prev_order(&self) -> Vec<K>{
        let mut res = Vec::new();
        let mut stack = VecDeque::new();
        let mut cur = self.root.as_ref();
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let node = cur.unwrap();
                res.push(node.key.clone());
                stack.push_back(node);
                cur = node.left.as_ref();
            }
            cur = stack.pop_back().and_then(|node| {
                node.right.as_ref()
            })
        }
        res
    }

    //中序遍历非迭代法
    fn in_order(&self) -> Vec<K>{
        let mut res = Vec::new();
        let mut stack = VecDeque::new();
        let mut cur = self.root.as_ref();
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let node = cur.unwrap();
                stack.push_back(node);
                cur = node.left.as_ref();
            }
            cur = stack.pop_back().and_then(|node| {
                res.push(node.key.clone());
                node.right.as_ref()
            })
        }
        res
    }

    //后序遍历非迭代法
    fn post_order(&self) -> Vec<K>{
        let mut res = Vec::new();
        let mut stack = VecDeque::new();
        let mut cur = self.root.as_ref();
        let mut prev: Option<&Box<Node<K, V>>> = None;
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let node = cur.unwrap();
                stack.push_back(node);
                cur = node.left.as_ref();
            }
            let top = stack.back().unwrap();
            if top.right.is_none() || top.right.as_ref() == prev {
                res.push(top.key.clone());
                prev = Some(top);
                let _ = stack.pop_back();
            }
            else {
                cur = top.right.as_ref();
            }
        }
        res
    }

    //层序遍历
    fn level_order(&self) -> Vec<K>{
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        if self.root.is_some() {
            queue.push_back(self.root.as_ref().unwrap());
        }
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                res.push(node.key.clone());
                if node.left.is_some() {
                    queue.push_back(node.left.as_ref().unwrap());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.as_ref().unwrap());
                }
            }
        }
        res
    }

    /// 前序遍历迭代器
    /// # Example
    /// ```
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(2, 'b');
    /// tree.insert(3, 'c');
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
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(2, 'b');
    /// tree.insert(3, 'c');
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
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(2, 'b');
    /// tree.insert(3, 'c');
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
    /// use an_ok_bstree::BSTree;
    /// let mut tree = BSTree::new();
    /// tree.insert(2, 'b');
    /// tree.insert(3, 'c');
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
}
