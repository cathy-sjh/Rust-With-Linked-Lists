use an_ok_stack::List as Stack;
use an_unsafe_queue::List as Queue;
use std::fmt::Debug;

#[derive(Clone)]
pub struct BinaryTree<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Eq, PartialEq, Clone)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
        }
    }
}

impl<T: Debug + PartialEq + Clone> BinaryTree<T> {
    //按照前序遍历创建二叉树
    pub fn new(val: &[T], invalid: T) -> Self {
        let mut index: usize = 0;
        Self {
            root: Self::create(val, &invalid,  &mut index)
        }
    }

    fn create(val: &[T], invalid: &T, index: &mut usize) -> Link<T> {
        let mut new_root: Link<T> = None;
        let cur = *index;
        if val[cur] != *invalid {
            new_root = Some(Box::new(Node::new(val[cur].clone())));

            *index += 1;
            if let Some(node) = new_root.as_mut() {
                node.left = Self::create(val, invalid, index);
            }

            *index += 1;
            if let Some(node) = new_root.as_mut() {
                node.right = Self::create(val, invalid, index);
            }
        }
        new_root
    }

    //前序遍历迭代法
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

    //前序遍历非迭代法
    pub fn prev_order_no_r(&self) -> Vec<T>{
        let mut res = Vec::new();
        let mut stack = Stack::new();
        let mut cur = self.root.as_ref();
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let node = cur.unwrap();
                res.push(node.elem.clone());
                stack.push(node);
                cur = node.left.as_ref();
            }
            cur = stack.pop().and_then(|node| {
                node.right.as_ref()
            })
        }
        res
    }

    //中序遍历迭代法
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

    //中序遍历非迭代法
    pub fn in_order_no_r(&self) -> Vec<T>{
        let mut res = Vec::new();
        let mut stack = Stack::new();
        let mut cur = self.root.as_ref();
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let node = cur.unwrap();
                stack.push(node);
                cur = node.left.as_ref();
            }
            cur = stack.pop().and_then(|node| {
                res.push(node.elem.clone());
                node.right.as_ref()
            })
        }
        res
    }

    //后序遍历迭代法
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

    //后序遍历非迭代法
    pub fn post_order_no_r(&self) -> Vec<T>{
        let mut res = Vec::new();
        let mut stack = Stack::new();
        let mut cur = self.root.as_ref();
        let mut prev: Option<&Box<Node<T>>> = None;
        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let node = cur.unwrap();
                stack.push(node);
                cur = node.left.as_ref();
            }
            let top = stack.peek().unwrap();
            if top.right.is_none() || top.right.as_ref() == prev {
                res.push(top.elem.clone());
                prev = Some(top);
                let _ = stack.pop();
            }
            else {
                cur = top.right.as_ref();
            }
        }
        res
    }

    //层序遍历
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

    //求总的节点个数
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

    //求叶子节点的个数
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

    //求二叉树的高度
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

    //求第K层的节点个数：
    pub fn get_level_node_size(&self, level: usize) -> Option<usize> {
        let height = self.tree_height();
        if level > height || level == 0 {
            return None;
        }
        Some(Self::get_level_node_size_help(&self.root, level))
    }

    fn get_level_node_size_help(root: &Link<T>, level: usize) -> usize{
        if root.is_none() {
            return 0;
        }
        if level == 1 {
            return 1;
        }
        let left = &root.as_ref().unwrap().left;
        let right = &root.as_ref().unwrap().right;
        Self::get_level_node_size_help(left, level - 1) + Self::get_level_node_size_help(right, level - 1)
    }

    //查找一个节点是否在一棵二叉树中
    pub fn find(&self, key: T) -> bool {
        Self::find_help(&self.root, key)
    }

    fn find_help(root: &Link<T>, key: T) -> bool {
        if root.is_none() {
            return false;
        }
        if root.as_ref().unwrap().elem == key {
            return true;
        }

        let left = &root.as_ref().unwrap().left;
        let right = &root.as_ref().unwrap().right;
        let res = Self::find_help(left, key.clone())
            || Self::find_help(right, key.clone());
        if res {
            return res;
        }
        false
    }

    //判断二叉树是否为完全二叉树
    pub fn is_complete_tree(&self) -> bool {
        let mut queue = Queue::new();
        if self.root.is_some() {
            queue.push(self.root.as_ref().unwrap());
        }
        let mut flag = true;
        while !queue.is_empty() {
            let head = queue.pop().unwrap();
            if head.left.is_some() {
                if !flag {
                    return false;
                }
                queue.push(head.left.as_ref().unwrap());
            }
            else {
                flag = false;
            }

            if head.right.is_some() {
                if !flag {
                    return false;
                }
                queue.push(head.right.as_ref().unwrap());
            }
            else {
                flag = false;
            }
        }
        true
    }

    pub fn destroy_tree(self) {
        Self::destroy_tree_help(self.root);
    }

    fn destroy_tree_help(root: Link<T>) {
        if root.is_none() {
            return;
        }
        let cur = root.unwrap();
        Self::destroy_tree_help(cur.left);
        Self::destroy_tree_help(cur.right);
    }
}

#[cfg(test)]
mod tests {
    use crate::BinaryTree;

    #[test]
    fn basic() {
        let array = ['A', 'B', '#', 'D', '#', '#', 'C' ,'#', '#'];
        let tree = BinaryTree::new(&array, '#');
        assert_eq!(tree.prev_orer(), vec!['A', 'B', 'D', 'C']);
        assert_eq!(tree.prev_order_no_r(), vec!['A', 'B', 'D', 'C']);
        assert_eq!(tree.in_order(), vec!['B', 'D', 'A', 'C']);
        assert_eq!(tree.in_order_no_r(), vec!['B', 'D', 'A', 'C']);
        assert_eq!(tree.post_order(), vec!['D', 'B', 'C', 'A']);
        assert_eq!(tree.post_order_no_r(), vec!['D', 'B', 'C', 'A']);
        assert_eq!(tree.level_order(), vec!['A', 'B', 'C', 'D']);

        let array = ["Alian".to_string(), "Bob".to_string(), "no".to_string()
            , "David".to_string(), "no".to_string(), "no".to_string(), "Clion".to_string()
            ,"no".to_string(), "no".to_string()];
        let tree = BinaryTree::new(&array, "no".to_string());
        assert_eq!(tree.prev_orer(), vec!["Alian", "Bob", "David", "Clion"]);
        assert_eq!(tree.prev_order_no_r(), vec!["Alian", "Bob", "David", "Clion"]);
        assert_eq!(tree.in_order(), vec!["Bob", "David", "Alian", "Clion"]);
        assert_eq!(tree.in_order_no_r(), vec!["Bob", "David", "Alian", "Clion"]);
        assert_eq!(tree.post_order(), vec!["David", "Bob", "Clion", "Alian"]);
        assert_eq!(tree.post_order_no_r(), vec!["David", "Bob", "Clion", "Alian"]);
        assert_eq!(tree.level_order(), vec!["Alian", "Bob", "Clion", "David"]);
    }

    /*
               1
              /   \
            2      3
           /      /  \
         4       5    6

    */
    #[test]
    fn size_test() {
        let array = [1, 2, 4,i32::MIN, i32::MIN, i32::MIN
            ,3, 5, i32::MIN, i32::MIN, 6, i32::MIN, i32::MIN];
        let tree = BinaryTree::new(&array, i32::MIN);
        assert_eq!(tree.prev_order_no_r(), vec![1, 2, 4, 3, 5, 6]);
        assert_eq!(tree.in_order_no_r(), vec![4,2,1,5,3,6]);
        assert_eq!(tree.post_order_no_r(), vec![4,2,5,6,3,1]);
        assert_eq!(tree.level_order(), vec![1,2,3,4,5,6]);

        assert_eq!(tree.tree_node_size(), 6);
        assert_eq!(tree.tree_leaf_size(), 3);
        assert_eq!(tree.tree_height(), 3);

        assert_eq!(tree.get_level_node_size(0), None);
        assert_eq!(tree.get_level_node_size(1), Some(1));
        assert_eq!(tree.get_level_node_size(2), Some(2));
        assert_eq!(tree.get_level_node_size(3), Some(3));
        assert_eq!(tree.get_level_node_size(4), None);

        assert_eq!(tree.find(0), false);
        assert_eq!(tree.find(1), true);
        assert_eq!(tree.find(2), true);
        assert_eq!(tree.find(3), true);
        assert_eq!(tree.find(4), true);
        assert_eq!(tree.find(5), true);
        assert_eq!(tree.find(6), true);
        assert_eq!(tree.find(7), false);

        assert_eq!(tree.is_complete_tree(), false);
    }

    #[test]
    fn is_complete_test() {
        let array = [1, 2, 4,i32::MIN, i32::MIN, 5, i32::MIN, i32::MIN
            ,3, 6, i32::MIN, i32::MIN, 7, i32::MIN, i32::MIN];
        let tree = BinaryTree::new(&array, i32::MIN);
        assert_eq!(tree.is_complete_tree(), true);
        tree.destroy_tree();
    }
}
