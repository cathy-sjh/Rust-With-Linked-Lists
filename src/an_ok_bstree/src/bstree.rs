use an_unsafe_queue::List as Queue;
use crate::{Link, Node};

pub struct BSTree<T> {
    root: Link<T>,
}

impl<T: PartialOrd + Clone> BSTree<T> {
    pub fn new() -> Self {
        BSTree {
            root: None
        }
    }

    ///查找是否存在值val
    pub fn search(&self, val: &T) -> bool {
        if let Some(ref node) = self.root {
            node.search(val)
        }
        else {
            return false;
        }
    }

    ///返回当前树中的最大值
    pub fn search_max(&self) -> Option<&T> {
        if let Some(ref node) = self.root {
            Some(node.search_max())
        }
        else {
            None
        }
    }

    ///返回当前树中的最小值
    pub fn search_min(&self) -> Option<&T> {
        if let Some(ref node) = self.root {
            Some(node.search_min())
        }
        else {
            None
        }
    }

    ///插入值，返回是否插入成功
    pub fn insert(&mut self, new_value: T) -> bool {
        match self.root {
            None => {
                self.root = Some(Box::new(Node::new(new_value)));
                true
            }
            Some(ref mut node) => {
                node.insert(new_value)
            }
        }
    }

    ///删除值为val的节点，并保持树仍为二叉搜索树
    pub fn delete(&mut self, val: T) -> bool {
        match self.root {
            None => {
                return false;
            }
            Some(ref mut node) if node.elem == val => {
                if node.is_leaf() {
                    self.root = None;
                    return true;
                }
                else {
                    node.delete_value()
                }
            }
            Some(ref mut node) => {
                if node.search(&val) {
                    node.delete(val)
                }
                else {
                    return false;
                }
            }
        }
    }

    /// 删除以val为根节点的树枝
    pub fn delete_tree(&mut self, val: T) -> bool {
        match self.root {
            None => {
                return false;
            }
            Some(ref mut node) if node.elem == val => {
                self.root = None;
                true
            }
            Some(ref mut node) => {
                node.delete_tree(val)
            }
        }
    }

    /// 删除以val为根节点的树枝, 并返回切掉的树枝
    pub fn remove_tree(&mut self, val: T) -> Self {
        let ret_node = match self.root {
            None => {
                None
            }
            Some(ref mut node) if node.elem == val => {
                self.root.take()
            }
            Some(ref mut node) => {
                node.remove_tree(val)
            }
        };
        Self {
            root: ret_node
        }
    }

    ///前序遍历
    pub fn prev_orer(&self) -> Vec<T>{
        let mut res = Vec::new();
        Self::prev_order_help(&self.root, &mut res);
        res
    }

    fn prev_order_help(root: &Link<T>, res: &mut Vec<T>){
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap();
        res.push(node.elem.clone());
        Self::prev_order_help(&node.left, res);
        Self::prev_order_help(&node.right, res);
    }

    ///中序遍历
    pub fn in_order(&self)  -> Vec<T> {
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
    pub fn post_order(&self) -> Vec<T>{
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
    pub fn level_order(&self) -> Vec<T>{
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

    fn tree_leaf_size_help(root: &Link<T>) -> usize{
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
        return if left_height > right_height {
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
        Iter {
            data: queue,
        }
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