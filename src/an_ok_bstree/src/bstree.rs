use crate::{Link, Node};
use an_unsafe_queue::List as Queue;
use std::option::Option::Some;

pub struct BSTree<T> {
    root: Link<T>,
}

impl<T: PartialOrd + Clone> Default for BSTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialOrd + Clone> BSTree<T> {
    pub fn new() -> Self {
        BSTree { root: None }
    }

    ///查找是否存在值val,迭代法
    pub fn search(&self, val: &T) -> bool {
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            if *val < node.elem {
                current = node.left.as_ref();
            } else if *val > node.elem {
                current = node.right.as_ref();
            } else {
                return true;
            }
        }
        false
    }

    ///查找是否存在值val,递归法
    pub fn search_r(&self, val: &T) -> bool {
        if let Some(ref node) = self.root {
            node.search_r(val)
        } else {
            false
        }
    }

    ///返回当前树中的最大值，迭代法
    pub fn search_max(&self) -> Option<&T> {
        let mut current = self.root.as_ref();
        let mut right = current.unwrap().right.as_ref();
        while let Some(node) = right {
            current = right;
            right = node.right.as_ref();
        }
        current.map(|node| &node.elem)
    }

    ///返回当前树中的最大值，递归法
    pub fn search_max_r(&self) -> Option<&T> {
        if let Some(ref node) = self.root {
            Some(node.search_max_r())
        } else {
            None
        }
    }

    ///返回当前树中的最小值，迭代法
    pub fn search_min(&self) -> Option<&T> {
        let mut current = self.root.as_ref();
        let mut left = current.unwrap().left.as_ref();
        while let Some(node) = left {
            current = left;
            left = node.left.as_ref();
        }
        current.map(|node| &node.elem)
    }

    ///返回当前树中的最小值，递归法
    pub fn search_min_r(&self) -> Option<&T> {
        if let Some(ref node) = self.root {
            Some(node.search_min_r())
        } else {
            None
        }
    }

    ///按中序遍历，查找val节点的直接后继
    pub fn successor(&self, val: &T) -> Option<&T> {
        let mut current = self.root.as_ref();
        let mut successor = None;
        while let Some(node) = current {
            if node.elem > *val {
                successor = current;
                current = node.left.as_ref();
            } else {
                current = node.right.as_ref();
            }
        }
        successor.map(|node| &node.elem)
    }

    ///按中序遍历，查找val节点的直接前躯
    pub fn predecessor(&self, val: &T) -> Option<&T> {
        let mut current = self.root.as_ref();
        let mut predecessor = None;
        while let Some(node) = current {
            if node.elem < *val {
                predecessor = current;
                current = node.right.as_ref();
            } else {
                current = node.left.as_ref();
            }
        }
        predecessor.map(|node| &node.elem)
    }

    ///插入值，返回是否插入成功，迭代法
    pub fn insert(&mut self, new_val: T) -> bool {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(new_val)));
            return true;
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current {
            if new_val < cur.elem {
                if cur.left.is_none() {
                    cur.left = Some(Box::new(Node::new(new_val)));
                    return true;
                } else {
                    current = cur.left.as_mut();
                }
            } else if new_val > cur.elem {
                if cur.right.is_none() {
                    cur.right = Some(Box::new(Node::new(new_val)));
                    return true;
                } else {
                    current = cur.right.as_mut();
                }
            } else {
                return false;
            }
        }
        false
    }

    ///插入值，返回是否插入成功，递归法
    pub fn insert_r(&mut self, new_value: T) -> bool {
        match self.root {
            None => {
                self.root = Some(Box::new(Node::new(new_value)));
                true
            }
            Some(ref mut node) => node.insert_r(new_value),
        }
    }

    ///删除值为val的节点，并保持树仍为二叉搜索树，迭代法
    pub fn delete(&mut self, val: T) -> bool {
        if self.root.is_none() {
            return false;
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current {
            if val < cur.elem {
                if let Some(left) = cur.left.as_mut() {
                    if left.elem == val && left.is_leaf() {
                        cur.left = None;
                        return true;
                    } else if left.elem == val {
                        return left.delete_value();
                    } else {
                        current = cur.left.as_mut();
                    }
                } else {
                    return false;
                }
            } else if val > cur.elem {
                if let Some(right) = cur.right.as_mut() {
                    if right.elem == val && right.is_leaf() {
                        cur.right = None;
                        return true;
                    } else if right.elem == val {
                        return right.delete_value();
                    } else {
                        current = cur.right.as_mut();
                    }
                } else {
                    return false;
                }
            } else if cur.is_leaf() {
                self.root.take();
                return true;
            } else {
                return cur.delete_value();
            }
        }
        false
    }

    ///删除值为val的节点，并保持树仍为二叉搜索树，递归法
    pub fn delete_r(&mut self, val: T) -> bool {
        match self.root {
            None => false,
            Some(ref mut node) if node.elem == val => {
                if node.is_leaf() {
                    self.root = None;
                    true
                } else {
                    node.delete_value()
                }
            }
            Some(ref mut node) => {
                if node.search_r(&val) {
                    node.delete_r(val)
                } else {
                    false
                }
            }
        }
    }

    /// 删除以val为根节点的树枝，迭代法
    pub fn delete_tree(&mut self, val: T) -> bool {
        if self.root.is_none() {
            return false;
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current {
            if val < cur.elem {
                if let Some(left) = cur.left.as_mut() {
                    if left.elem == val {
                        cur.left.take();
                        return true;
                    } else {
                        current = cur.left.as_mut();
                    }
                } else {
                    return false;
                }
            } else if val > cur.elem {
                if let Some(right) = cur.right.as_mut() {
                    if right.elem == val {
                        cur.right.take();
                        return true;
                    } else {
                        current = cur.right.as_mut();
                    }
                } else {
                    return false;
                }
            } else {
                self.root.take();
                return true;
            }
        }
        false
    }

    /// 删除以val为根节点的树枝，递归法
    pub fn delete_tree_r(&mut self, val: T) -> bool {
        match self.root {
            None => false,
            Some(ref mut node) if node.elem == val => {
                self.root = None;
                true
            }
            Some(ref mut node) => node.delete_tree_r(val),
        }
    }

    pub fn remove_tree(&mut self, val: T) -> Self {
        if self.root.is_none() {
            return Self::new();
        }
        let mut current = self.root.as_mut();
        while let Some(cur) = current {
            if val < cur.elem {
                if let Some(left) = cur.left.as_mut() {
                    if left.elem == val {
                        return Self {
                            root: cur.left.take(),
                        };
                    } else {
                        current = cur.left.as_mut();
                    }
                } else {
                    return Self::new();
                }
            } else if val > cur.elem {
                if let Some(right) = cur.right.as_mut() {
                    if right.elem == val {
                        return Self {
                            root: cur.right.take(),
                        };
                    } else {
                        current = cur.right.as_mut();
                    }
                } else {
                    return Self::new();
                }
            } else {
                return Self {
                    root: self.root.take(),
                };
            }
        }
        Self::new()
    }

    /// 删除以val为根节点的树枝, 并返回切掉的树枝，递归法
    pub fn remove_tree_r(&mut self, val: T) -> Self {
        let ret_node = match self.root {
            None => None,
            Some(ref mut node) if node.elem == val => self.root.take(),
            Some(ref mut node) => node.remove_tree_r(val),
        };
        Self { root: ret_node }
    }

    ///前序遍历
    pub fn prev_orer(&self) -> Vec<T> {
        let mut res = Vec::new();
        Self::prev_order_help(&self.root, &mut res);
        res
    }

    fn prev_order_help(root: &Link<T>, res: &mut Vec<T>) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap();
        res.push(node.elem.clone());
        Self::prev_order_help(&node.left, res);
        Self::prev_order_help(&node.right, res);
    }

    ///中序遍历
    pub fn in_order(&self) -> Vec<T> {
        let mut res = Vec::new();
        Self::in_order_help(&self.root, &mut res);
        res
    }

    fn in_order_help(root: &Link<T>, res: &mut Vec<T>) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap();
        Self::in_order_help(&node.left, res);
        res.push(node.elem.clone());
        Self::in_order_help(&node.right, res);
    }

    ///后序遍历
    pub fn post_order(&self) -> Vec<T> {
        let mut res = Vec::new();
        Self::post_order_help(&self.root, &mut res);
        res
    }

    fn post_order_help(root: &Link<T>, res: &mut Vec<T>) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap();
        Self::post_order_help(&node.left, res);
        Self::post_order_help(&node.right, res);
        res.push(node.elem.clone());
    }

    ///层序遍历
    pub fn level_order(&self) -> Vec<T> {
        let mut res = Vec::new();
        let mut queue = Queue::new();
        if self.root.is_some() {
            queue.push(self.root.as_ref().unwrap());
        }
        while !queue.is_empty() {
            if let Some(node) = queue.pop() {
                res.push(node.elem.clone());
                if node.left.is_some() {
                    queue.push(node.left.as_ref().unwrap());
                }
                if node.right.is_some() {
                    queue.push(node.right.as_ref().unwrap());
                }
            }
        }
        res
    }

    ///求二叉树总的节点个数
    pub fn tree_node_size(&self) -> usize {
        let mut size: usize = 0;
        Self::tree_node_size_help(&self.root, &mut size);
        size
    }

    fn tree_node_size_help(root: &Link<T>, size: &mut usize) {
        if root.is_none() {
            return;
        }
        Self::tree_node_size_help(&root.as_ref().unwrap().left, size);
        *size += 1;
        Self::tree_node_size_help(&root.as_ref().unwrap().right, size);
    }

    ///求二叉树叶子节点的个数
    pub fn tree_leaf_size(&self) -> usize {
        Self::tree_leaf_size_help(&self.root)
    }

    fn tree_leaf_size_help(root: &Link<T>) -> usize {
        if root.is_none() {
            return 0;
        }
        if root.as_ref().unwrap().left.is_none() && root.as_ref().unwrap().right.is_none() {
            return 1;
        }
        let left = &root.as_ref().unwrap().left;
        let right = &root.as_ref().unwrap().right;
        Self::tree_leaf_size_help(left) + Self::tree_leaf_size_help(right)
    }

    ///求二叉树的高度
    pub fn tree_height(&self) -> usize {
        Self::tree_height_help(&self.root)
    }

    fn tree_height_help(root: &Link<T>) -> usize {
        if root.is_none() {
            return 0;
        }
        let left_height = Self::tree_height_help(&root.as_ref().unwrap().left);
        let right_height = Self::tree_height_help(&root.as_ref().unwrap().right);
        if left_height > right_height {
            left_height + 1
        } else {
            right_height + 1
        }
    }

    ///树是否为空
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn iter(&self) -> Iter<T> {
        let res = self.in_order();
        let mut queue = Queue::new();
        for dat in res {
            queue.push(dat);
        }
        Iter { data: queue }
    }
}

pub struct Iter<T> {
    data: Queue<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}
