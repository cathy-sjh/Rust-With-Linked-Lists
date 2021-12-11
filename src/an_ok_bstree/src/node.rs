
pub type Link<T> = Option<Box<Node<T>>>;
pub struct Node<T> {
    pub elem: T,
    pub left: Link<T>,
    pub right: Link<T>,
}

impl<T: PartialOrd + Clone> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    //找到当前树的最小节点值T,移除以T为根节点的子树
    pub fn remove_min_node(&mut self) -> Link<T> {
        if let Some(ref mut node) = self.left {
            if node.left.is_none() {
                self.left.take()
            }
            else {
                node.remove_min_node()
            }
        }
        else {
            None
        }
    }

    //找到当前树的最大节点值T,移除以T为根节点的子树
    pub fn remove_max_node(&mut self) -> Link<T> {
        if let Some(ref mut node) = self.right {
            if node.right.is_none() {
                self.right.take()
            }
            else {
                node.remove_max_node()
            }
        }
        else {
            None
        }
    }

    //获得当前树的最大节点的可变借用，不存在则返回当前节点，有可能违反二叉排序树规则
    fn get_max_node_mut(&mut self) -> &mut Node<T> {
        if let Some(ref mut node) = self.right {
            node.get_max_node_mut()
        }
        else {
            self
        }
    }

    //获得当前树的最小节点的可变借用，不存在则返回当前节点，有可能违反二叉排序树规则
    fn get_min_node_mut(&mut self) -> &mut Node<T> {
        if let Some(ref mut node) = self.left {
            node.get_min_node_mut()
        }
        else {
            self
        }
    }


    pub fn insert(&mut self, new_value: T) -> bool{
        if self.search(&new_value) {
            return false;
        }
        if self.elem > new_value {
            match self.left {
                None => {
                    self.left = Some(Box::new(Node::new(new_value)));
                    true
                }
                Some(ref mut node) => {
                    node.insert(new_value)
                }
            }
        }
        else {
            match self.right {
                None => {
                    self.right = Some(Box::new(Node::new(new_value)));
                    true
                }
                Some(ref mut node) => {
                    node.insert(new_value)
                }
            }
        }
    }

    pub fn search(&self, val: &T) -> bool {
        if self.elem == *val {
            return true;
        }

        if self.elem > *val {
            match self.left {
                None => {
                    return false;
                }
                Some(ref node) => {
                    node.search(val)
                }
            }
        }
        else {
            match self.right {
                None => {
                    return false;
                }
                Some(ref node) => {
                    node.search(val)
                }
            }
        }
    }

    pub fn search_max(&self) -> &T {
        if let Some(ref right) = self.right {
            right.search_max()
        }
        else {
            &self.elem
        }
    }

    pub fn search_min(&self) -> &T {
        if let Some(ref left) = self.left {
            left.search_min()
        }
        else {
            &self.elem
        }
    }

    // 将当前节点替换为别的节点，并调整二叉树，使之保持二叉搜索树结构。
    // 无法调整叶子节点
    pub fn delete_value(&mut self) -> bool {
        match (&mut self.left, &mut self.right) {
            (Some(left), _) => {
                if left.is_leaf() {
                    self.elem = left.elem.clone();
                    self.left = None;
                }
                else {
                    //successor是以left的最大节点为根节点的子树，要么包含一个节点，要么包含两个节点
                    if let Some(mut successor) = left.remove_max_node() {
                        let successor_node_to_insert = successor.get_min_node_mut();
                        successor_node_to_insert.left = self.left.take();
                        successor.right = self.right.take();
                        *self = *successor;
                    }
                    else { //left没有右子树
                        left.right = self.right.take();
                        *self = *self.left.take().unwrap();
                    }
                }
                return true;
            }

            (_, Some(right)) => {
                if right.is_leaf() {
                    self.elem = right.elem.clone();
                    self.right = None;
                    return true;
                }
                else {
                    //successor是以right的最小节点为根节点的子树，要么一个节点，要么两个节点
                    if let Some(mut successor) = right.remove_min_node() {
                        let successor_node_to_insert = successor.get_max_node_mut();
                        successor_node_to_insert.right = self.right.take();
                        successor.left = self.left.take();
                        *self = *successor;
                    }
                    else { //right没有左子树
                        right.left = self.left.take();
                        *self = *self.right.take().unwrap();
                    }
                }
                return true;
            }
            (_, _) => {
                return false;
            }
        }
    }

    // 删除值为val的节点
    // 无法直接删除根节点
    pub fn delete(&mut self, val: T) -> bool {
        match self {
            Node {
                elem: value,
                left: Some(left),
                ..
            } if val < *value => {
                if left.elem == val && left.is_leaf() {
                    self.left = None;
                    return true;
                }
                else if left.elem == val {
                    left.delete_value()
                }
                else {
                    left.delete(val)
                }
            }

            Node {
                elem: value,
                right: Some(right),
                ..
            } if val > *value => {
                if right.elem == val && right.is_leaf() {
                    self.right = None;
                    return true;
                }
                else if right.elem == val {
                    right.delete_value()
                }
                else {
                    right.delete(val)
                }
            }
            _ => {
                return false;
            }
        }
    }

    // 删除以val为根节点的树枝
    // 无法直接删除根节点
    pub fn delete_tree(&mut self, val: T) -> bool {
        match self {
           Node {
               elem: value,
               left: Some(left),
               ..
           } if val < *value => {
                if left.elem == val {
                    self.left = None;
                    true
                }
               else {
                   left.delete_tree(val)
               }
           }

            Node {
                elem: value,
                right: Some(right),
                ..
            } if val > *value => {
                if right.elem == val {
                    self.right = None;
                    true
                }
                else {
                    right.delete_tree(val)
                }
            }
            _ => {
                false
            }
        }
    }

    // 删除以val为根节点的树枝, 并返回切掉的树枝
    // 无法直接删除根节点
    pub fn remove_tree(&mut self, val: T) -> Link<T> {
        match self {
            Node {
                elem: value,
                left: Some(left),
                ..
            } if val < *value => {
                if left.elem == val {
                    self.left.take()
                }
                else {
                    left.remove_tree(val)
                }
            }

            Node {
                elem: value,
                right: Some(right),
                ..
            } if val > *value => {
                if right.elem == val {
                    self.right.take()
                }
                else {
                    right.remove_tree(val)
                }
            }

            _ => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;

    #[test]
    fn basic() {
        let mut node = Node::new(0);
        assert_eq!(node.insert(10), true);
        assert_eq!(node.insert(2), true);
        assert_eq!(node.insert(30), true);
        assert_eq!(node.insert(2), false);
        assert_eq!(node.search(&10), true);
        assert_eq!(node.search(&2), true);
        assert_eq!(node.search(&30), true);
        assert_eq!(node.search(&4), false);
        assert_eq!(node.elem, 0);

        assert_eq!(node.remove_max_node().unwrap().elem, 30);
        assert_eq!(node.search(&30), false);
        assert_eq!(node.remove_max_node().unwrap().elem, 10);
        assert_eq!(node.search(&10), false);
        assert_eq!(node.search(&2), false);

        assert!(node.remove_min_node().is_none());
    }

    #[test]
    fn get_mut() {
        let mut node = Node::new(0);
        assert_eq!(node.insert(10), true);
        assert_eq!(node.insert(2), true);
        assert_eq!(node.insert(30), true);
        assert_eq!(node.insert(2), false);
        assert_eq!(node.search(&30), true);
        node.get_max_node_mut().elem = 50;
        assert_eq!(node.search(&30), false);

        assert_eq!(node.search(&0), true);
        node.get_min_node_mut().elem = 1;
        assert_eq!(node.search(&0), false);
    }

    #[test]
    fn delete() {
        let mut node = Node::new(0);
        assert_eq!(node.insert(10), true);
        assert_eq!(node.insert(2), true);
        assert_eq!(node.insert(30), true);

        assert_eq!(node.delete(10), true);
        assert_eq!(node.search(&10), false);
        assert_eq!(node.search(&2), true);
        assert_eq!(node.search(&30), true);
        assert_eq!(node.search(&0), true);
    }
}