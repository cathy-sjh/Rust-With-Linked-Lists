use crate::Queue;
use std::cmp::max;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    key: K,
    value: V,
    height: u32,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: PartialOrd, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl<K: PartialOrd + ToString, V: ToString> ToString for Node<K, V> {
    fn to_string(&self) -> String {
        format!(
            "[K: {}, V: {}, L: {}, R: {}]",
            self.key.to_string(),
            self.value.to_string(),
            to_string(&self.left),
            to_string(&self.right)
        )
    }
}

pub fn to_string<K: PartialOrd + ToString, V: ToString>(node: &Link<K, V>) -> String {
    match node {
        None => "Ø".to_string(),
        Some(box_node) => box_node.to_string(),
    }
}

fn height<K, V>(node: &Link<K, V>) -> u32 {
    node.as_ref().map_or(0, |node| node.height)
}

fn update_height<K, V>(root: &mut Node<K, V>) {
    root.height = max(height(&root.left), height(&root.right)) + 1;
}

//对当前节点进行一次左旋操作，返回旋转后的根节点
fn left_rotate<K, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut new_root = root.right.take().expect("AVL broken");
    root.right = new_root.left.take();
    update_height(&mut root);
    new_root.left = Some(root);
    update_height(&mut new_root);
    new_root
}

//对当前节点进行一次右旋操作，返回旋转后的根节点
fn right_rotate<K, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut new_root = root.left.take().expect("AVL broken");
    root.left = new_root.right.take();
    update_height(&mut root);
    new_root.right = Some(root);
    update_height(&mut new_root);
    new_root
}

//保持左侧平衡。传入的root是一颗不平衡的树，左子树比右子树高2
fn left_balance<K, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let left = root.left.take().expect("AVL broken");
    if height(&left.left) < height(&left.right) {
        let rotated = left_rotate(left);
        root.left = Some(rotated);
        update_height(&mut root);
    } else {
        root.left = Some(left);
    }
    right_rotate(root)
}

//保持右侧平衡。传入的root是一颗不平衡的树，右子树比左子树高2
fn right_balance<K, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let right = root.right.take().expect("AVL broken");
    if height(&right.left) > height(&right.right) {
        let rotated = right_rotate(right);
        root.right = Some(rotated);
        update_height(&mut root);
    } else {
        root.right = Some(right);
    }
    left_rotate(root)
}

//计算当前节点左右子树的高度差
fn diff_of_height<K, V>(root: &Node<K, V>) -> i32 {
    let l = height(&root.left);
    let r = height(&root.right);
    (l as i32) - (r as i32)
}

//判断当前节点是否需要进行旋转调整，返回调整后的根节点
fn rotate_if_necessary<K, V>(root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let diff = diff_of_height(&root);
    if -1 <= diff && diff <= 1 {
        root
    } else if diff == -2 {
        right_balance(root)
    } else if diff == 2 {
        left_balance(root)
    } else {
        unreachable!()
    }
}

//更新当前根节点
fn update_node<K: PartialOrd, V>(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    update_height(&mut *root);
    rotate_if_necessary(root)
}

//插入新节点，并返回调整后的根节点
pub fn insert<K: PartialOrd, V>(key: K, value: V, mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    if root.key > key {
        match root.left.take() {
            None => {
                root.left = Some(Box::new(Node::new(key, value)));
            }
            Some(node) => {
                root.left = Some(insert(key, value, node));
            }
        }
    } else if root.key < key {
        match root.right.take() {
            None => {
                root.right = Some(Box::new(Node::new(key, value)));
            }
            Some(node) => {
                root.right = Some(insert(key, value, node));
            }
        }
    } else {
        root.value = value;
        return root;
    }
    update_node(root)
}

//找出当前树中值最小的节点，返回元组(除去最小节点后剩下的树(满足AVL规则)，最小节点)
fn remove_min<K: PartialOrd, V>(mut root: Box<Node<K, V>>) -> (Link<K, V>, Box<Node<K, V>>) {
    match root.left.take() {
        Some(left) => {
            let (new_left, min) = remove_min(left);
            root.left = new_left;
            (Some(update_node(root)), min)
        }
        None => (root.right.take(), root),
    }
}

//将两棵子树合并为一棵，合并后仍然满足AVL树的规则，返回新生成树的根节点
fn combine_two_subtrees<K: PartialOrd, V>(
    left: Box<Node<K, V>>,
    right: Box<Node<K, V>>,
) -> Box<Node<K, V>> {
    let (remain_tree, min) = remove_min(right);
    let mut new_root = min;
    new_root.right = remain_tree;
    new_root.left = Some(left);
    update_node(new_root)
}

//删除根节点，重构二叉树，并返回新的根节点
fn delete_root<K: PartialOrd, V>(mut root: Node<K, V>) -> Link<K, V> {
    match (root.left.take(), root.right.take()) {
        (None, None) => None,
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (Some(left), Some(right)) => Some(combine_two_subtrees(left, right)),
    }
}

//删除节点key，并保持改树仍为AVL树，返回的树的根节点
pub fn delete<K: PartialOrd, V>(key: K, mut root: Box<Node<K, V>>) -> Link<K, V> {
    if root.key < key {
        if let Some(succ) = root.right.take() {
            root.right = delete(key, succ);
            return Some(update_node(root));
        }
    } else if root.key > key {
        if let Some(succ) = root.left.take() {
            root.left = delete(key, succ);
            return Some(update_node(root));
        }
    } else {
        return delete_root(*root);
    }
    Some(root)
}

//按中序遍历，查找val节点的直接后继
pub fn successor<'a, K: PartialOrd, V>(key: &K, root: &'a Node<K, V>) -> Option<(&'a K, &'a V)> {
    if root.key > *key {
        match root.left {
            None => Some((&root.key, &root.value)),
            Some(ref succ) => successor(key, succ).or(Some((&root.key, &root.value))),
        }
    } else if root.key < *key {
        root.right.as_ref().and_then(|right| successor(key, right))
    } else {
        root.right.as_ref().map(|right| min_pair(right))
    }
}

//按中序遍历，查找val节点的直接前躯
pub fn predecessor<'a, K: PartialOrd, V>(key: &K, root: &'a Node<K, V>) -> Option<(&'a K, &'a V)> {
    if root.key < *key {
        match root.right {
            None => Some((&root.key, &root.value)),
            Some(ref succ) => predecessor(key, succ).or(Some((&root.key, &root.value))),
        }
    } else if root.key > *key {
        root.left.as_ref().and_then(|left| predecessor(key, left))
    } else {
        root.left.as_ref().map(|left| max_pair(left))
    }
}

//前序遍历
pub fn prev_order<K: PartialOrd + Clone, V>(root: &Link<K, V>, buf: &mut Vec<K>) {
    if let Some(node) = root {
        buf.push(node.key.clone());
        prev_order(&node.left, buf);
        prev_order(&node.right, buf);
    }
}

//中序遍历
pub fn in_order<K: PartialOrd + Clone, V>(root: &Link<K, V>, buf: &mut Vec<K>) {
    if let Some(node) = root {
        in_order(&node.left, buf);
        buf.push(node.key.clone());
        in_order(&node.right, buf);
    }
}

//后序遍历
pub fn post_order<K: PartialOrd + Clone, V>(root: &Link<K, V>, buf: &mut Vec<K>) {
    if let Some(node) = root {
        post_order(&node.left, buf);
        post_order(&node.right, buf);
        buf.push(node.key.clone());
    }
}

//层序遍历
pub fn level_order<K: PartialOrd + Clone, V>(root: &Link<K, V>, buf: &mut Vec<K>) {
    let mut queue = Queue::new();
    if let Some(node) = root {
        queue.push(node);
    }
    while !queue.is_empty() {
        if let Some(node) = queue.pop() {
            buf.push(node.key.clone());
            if let Some(left) = node.left.as_ref() {
                queue.push(left);
            }
            if let Some(right) = node.right.as_ref() {
                queue.push(right);
            }
        }
    }
}

//返回查找的键值对的不可变借用
pub fn search_pair<'a, K: PartialOrd, V>(key: &K, root: &'a Node<K, V>) -> Option<(&'a K, &'a V)> {
    if root.key < *key {
        root.right
            .as_ref()
            .and_then(|right| search_pair(key, right))
    } else if root.key > *key {
        root.left.as_ref().and_then(|left| search_pair(key, left))
    } else {
        Some((&root.key, &root.value))
    }
}

//根据键查找对应的值
pub fn search<'a, K: PartialOrd, V>(key: &K, root: &'a Node<K, V>) -> Option<&'a V> {
    search_pair(key, root).map(|(_, v)| v)
}

//返回AVL树中的最小键值对
pub fn min_pair<K: PartialOrd, V>(root: &Node<K, V>) -> (&K, &V) {
    root.left
        .as_ref()
        .map_or((&root.key, &root.value), |left| min_pair(left))
}

//返回AVL树中的最大键值对
pub fn max_pair<K: PartialOrd, V>(root: &Node<K, V>) -> (&K, &V) {
    root.right
        .as_ref()
        .map_or((&root.key, &root.value), |right| max_pair(right))
}

//判断节点是否满足AVL树的性质
fn is_avl_node<K: PartialOrd, V>(node: &Node<K, V>) -> bool {
    if node.is_leaf() {
        return true;
    }
    if !node.left.as_ref().map_or(true, |succ| succ.key < node.key) {
        return false;
    }
    if !node.right.as_ref().map_or(true, |succ| succ.key > node.key) {
        return false;
    }
    let balance = diff_of_height(node);
    if balance > 1 || balance < -1 {
        return false;
    }
    true
}

//判断是否为AVL树
pub fn is_avl_tree<K: PartialOrd, V>(root: &Link<K, V>) -> bool {
    match root {
        None => true,
        Some(node) => {
            if !is_avl_node(node) {
                return false;
            }
            is_avl_tree(&node.left) && is_avl_tree(&node.right)
        }
    }
}
