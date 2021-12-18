use std::collections::VecDeque;

pub type Link<K, V> = Option<Box<Node<K, V>>>;
pub struct Node<K, V> {
    pub key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: PartialOrd + Clone, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }

    // 插入键值对
    pub fn insert(&mut self, key: K, value: V) {
        if self.key > key {
            match self.left {
                None => {
                    self.left = Some(Box::new(Node::new(key, value)));
                }
                Some(ref mut node) => node.insert(key, value),
            }
        } else if self.key < key {
            match self.right {
                None => {
                    self.right = Some(Box::new(Node::new(key, value)));
                }
                Some(ref mut node) => node.insert(key, value),
            }
        } else {
            self.value = value;
        }
    }

    // 返回查找的键值对的不可变借用
    pub fn search_pair(&self, key: &K,) -> Option<(&K, &V)> {
        if self.key < *key {
            self.right
                .as_ref()
                .and_then(|right| right.search_pair(key))
        } else if self.key > *key {
            self.left.as_ref().and_then(|left| left.search_pair(key))
        } else {
            Some((&self.key, &self.value))
        }
    }

    // 根据键查找对应的值
    pub fn search(&self, key: &K) -> Option<&V> {
        self.search_pair(key).map(|(_, v)| v)
    }

    // 返回AVL树中的最小键值对
    pub fn min_pair(&self) -> (&K, &V) {
        if let Some(ref left) = self.left {
            left.min_pair()
        } else {
            (&self.key, &self.value)
        }
    }

    // 返回AVL树中的最大键值对
    pub fn max_pair(&self) -> (&K, &V) {
        if let Some(ref right) = self.right {
            right.max_pair()
        } else {
            (&self.key, &self.value)
        }
    }

    // 返回第一个大于key的键值对,key可以不存在树中
    pub fn successor(&self, key: &K) -> Option<(&K, &V)> {
        if self.key > *key {
            match self.left {
                None => Some((&self.key, &self.value)),
                Some(ref succ) => succ.successor(key).or(Some((&self.key, &self.value))),
            }
        } else if self.key < *key {
            self.right.as_ref().and_then(|right| right.successor(key))
        } else {
            self.right.as_ref().map(|right| right.min_pair())
        }
    }

    // 返回第一个小于key的键值对,key可以不存在树中
    pub fn predecessor(&self, key: &K) -> Option<(&K, &V)> {
        if self.key < *key {
            match self.right {
                None => Some((&self.key, &self.value)),
                Some(ref succ) => succ.predecessor(key).or(Some((&self.key, &self.value))),
            }
        } else if self.key > *key {
            self.left.as_ref().and_then(|left| left.predecessor(key))
        } else {
            self.left.as_ref().map(|left| left.max_pair())
        }
    }

    //找出当前树中值最小的节点，返回元组:(除去最小节点后剩下的树，最小节点)
    fn remove_min(mut self) -> (Link<K, V>, Box<Node<K, V>>) {
        match self.left.take() {
            Some(left) => {
                let (new_left, min) = left.remove_min();
                self.left = new_left;
                (Some(Box::new(self)), min)
            }
            None => (self.right.take(), Box::new(self)),
        }
    }

    //将两棵子树合并为一棵，返回新生成树的根节点
    fn combine_two_subtrees(
        left: Node<K, V>,
        right: Node<K, V>,
    ) -> Box<Node<K, V>> {
        // 得到右子树中最小的节点和去除最小节点后剩余的树
        let (remain_tree, min) = right.remove_min();
        // 最小节点作为两个子树的新根节点
        let mut new_root = min;
        new_root.right = remain_tree;
        new_root.left = Some(Box::new(left));
        new_root
    }

    //删除当前节点，并返回新的根节点
    pub fn delete_root(mut self) -> Link<K, V> {
        // 二叉搜索树树删除节点的三种情况：
        // 1.如果是叶子节点，则直接删除
        // 2.如果待删除节点只有左子树或只有右子树，删除该节点，然后将左子树或右子树移动到该节点
        // 3.如果待删除节点左右子树都有，就选取右子树中最小的节点代替待删除节点的位置(或者取左子树中最大节点代替也可以)。
        match (self.left.take(), self.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => Some(Self::combine_two_subtrees(*left, *right)),
        }
    }

    //删除节点key，返回的新的根节点
    pub fn delete(mut self, key: K) -> Link<K, V> {
        if self.key < key {
            if let Some(right) = self.right.take() {
                self.right = right.delete(key);
                return Some(Box::new(self));
            }
        } else if self.key > key {
            if let Some(left) = self.left.take() {
                self.left = left.delete(key);
                return Some(Box::new(self));
            }
        }
        else {
            return self.delete_root()
        }
        Some(Box::new(self))
    }

    // 删除以key为根节点的树枝,无法直接删除根节点
    pub fn delete_tree(&mut self, key: K) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                if right.key == key {
                    self.right = None;
                }
                else {
                    right.delete_tree(key);
                }
            }
        }
        else if self.key > key {
            if let Some(ref mut left) = self.left {
                if left.key == key {
                    self.left = None;
                }
                else {
                    left.delete_tree(key);
                }
            }
        }
    }

    // 删除以key为根节点的树枝, 并返回切掉的树枝
    // 无法直接删除根节点
    pub fn remove_tree(&mut self, key: K) -> Link<K, V> {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                if right.key == key {
                    return self.right.take();
                }
                else {
                    return right.remove_tree(key);
                }
            }
        }
        else if self.key > key {
            if let Some(ref mut left) = self.left {
                if left.key == key {
                    return self.left.take();
                }
                else {
                    return left.remove_tree(key);
                }
            }
        }
        None
    }

    // 前序遍历
    pub fn prev_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        if let Some(node) = root {
            buf.push(node.key.clone());
            Self::prev_order(&node.left, buf);
            Self::prev_order(&node.right, buf);
        }
    }

    // 中序遍历
    pub fn in_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        if let Some(node) = root {
            Self::in_order(&node.left, buf);
            buf.push(node.key.clone());
            Self::in_order(&node.right, buf);
        }
    }

    // 后序遍历
    pub fn post_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        if let Some(node) = root {
            Self::post_order(&node.left, buf);
            Self::post_order(&node.right, buf);
            buf.push(node.key.clone());
        }
    }

    // 层序遍历
    pub fn level_order(root: &Link<K, V>, buf: &mut Vec<K>) {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                buf.push(node.key.clone());
                if let Some(left) = node.left.as_ref() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.as_ref() {
                    queue.push_back(right);
                }
            }
        }
    }
}

