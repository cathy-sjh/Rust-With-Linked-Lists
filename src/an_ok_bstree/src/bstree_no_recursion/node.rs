
pub type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub left: Link<K, V>,
    pub right: Link<K, V>,
}

impl<K: PartialEq, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
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

    //找出当前树中值最小的节点，返回元组:(除去最小节点后剩下的树，最小节点)
    fn remove_min(mut self) -> (Link<K, V>, Box<Node<K, V>>) {
        let mut current = &mut self;
        while let Some(mut left) = current.left.take() {
                if left.left.is_none() {
                    current.left = left.right.take();
                    return (Some(Box::new(self)), left);
                }
                else {
                    current.left = Some(left);
                    current = current.left.as_mut().unwrap();
                }

        }
        (self.right.take(), Box::new(self))
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
}

