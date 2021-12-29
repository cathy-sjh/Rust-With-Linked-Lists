use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

pub struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    degree: usize,
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BNode")
            .field(&self.keys)
            .field(&self.children)
            .finish()
    }
}

impl<T: PartialOrd + Clone + Debug> Node<T> {
    pub fn new(degree: usize, _key: Option<Vec<T>>, _child: Option<Vec<Node<T>>>) -> Self {
        let new_key = match _key {
            None => Vec::with_capacity(2 * degree - 1),
            Some(key) => key,
        };
        let new_child = match _child {
            None => Vec::with_capacity(2 * degree),
            Some(child) => child,
        };
        Node {
            keys: new_key,
            children: new_child,
            degree,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn is_full_keys(&self) -> bool {
        let max_keys = 2 * self.degree - 1;
        self.key_len() == max_keys
    }

    pub fn insert_child(&mut self, index: usize, elem: Node<T>) {
        self.children.insert(index, elem);
    }

    pub fn get_child(&self, index: usize) -> &Node<T> {
        &self.children[index]
    }

    pub fn insert_key(&mut self, index: usize, elem: T) {
        self.keys.insert(index, elem);
    }

    pub fn get_key(&self, index: usize) -> &T {
        &self.keys[index]
    }

    pub fn key_len(&self) -> usize {
        self.keys.len()
    }

    pub fn children_len(&self) -> usize {
        self.children.len()
    }

    // 分裂B树中child_index指向的孩子节点，输入必须满足：
    // 1.当前节点self是非满的内部节点，且不能是叶子节点
    // 2.下标为child_index的孩子是满节点
    pub fn split_child(&mut self, child_index: usize) {
        let mid_key_index = (2 * self.degree - 1) / 2;
        let child = &mut self.children[child_index];
        let right_keys = child.keys.split_off(mid_key_index + 1);
        let middle_key = child.keys.pop().unwrap();

        let right_child = if !child.is_leaf() {
            Some(child.children.split_off(mid_key_index + 1))
        } else {
            None
        };
        let new_child_node = Node::new(self.degree, Some(right_keys), right_child);
        self.insert_key(child_index, middle_key);
        self.insert_child(child_index + 1, new_child_node);
    }

    // 辅助的递归过程，将关键字key递归插入当前节点，调用时满足：
    // 1.当前节点self必须是非满的
    // 2.递归调用时如果发现孩子节点是满节点，则调用split_child()进行拆分
    pub fn insert_non_full(&mut self, key: T) {
        let mut i = isize::try_from(self.key_len()).ok().unwrap() - 1;
        while i >= 0 && self.keys[i as usize] >= key {
            i -= 1;
        }
        let mut u_index = usize::try_from(i + 1).ok().unwrap();
        if self.is_leaf() {
            self.insert_key(u_index, key);
        } else {
            if self.children[u_index].is_full_keys() {
                self.split_child(u_index);
                if self.keys[u_index] < key {
                    u_index += 1;
                }
            }
            self.children[u_index].insert_non_full(key);
        }
    }

    pub fn max_key(&self) -> T {
        let mut cur = self;
        while !cur.is_leaf() {
            cur = &cur.children[cur.key_len()];
        }
        cur.keys[cur.key_len() - 1].clone()
    }

    pub fn min_key(&self) -> T {
        let mut cur = self;
        while !cur.is_leaf() {
            cur = &cur.children[0];
        }
        cur.keys[0].clone()
    }

    pub fn delete(&mut self, key: T) {
        let t = self.degree; // 除了根节点外每个节点必须至少有t-1个关键字key
        let mut i = 0;
        while i < self.key_len() && self.keys[i] < key {
            i += 1;
        }
        if self.is_leaf() {
            //case 1
            if i < self.key_len() && key == self.keys[i] {
                self.keys.remove(i);
            }
            return;
        }
        if i < self.key_len() && key == self.keys[i] {
            //case 2
            if self.children[i].key_len() >= t {
                //case a
                let max_key = self.children[i].max_key();
                self.keys[i] = max_key.clone();
                return self.children[i].delete(max_key);
            } else if self.children[i + 1].key_len() >= t {
                //case b
                let min_key = self.children[i + 1].min_key();
                self.keys[i] = min_key.clone();
                return self.children[i + 1].delete(min_key);
            } else {
                // case c
                self.unionchild(i);
                return self.delete(key);
            }
        } else if self.children[i].key_len() == t - 1 {
            //case 3
            if i >= 1 && self.children[i - 1].key_len() >= t {
                // a_left
                self.roright(i);
                return self.children[i].delete(key);
            } else if i + 1 < self.children_len() && self.children[i + 1].key_len() >= t {
                // a_right
                self.roleft(i);
                return self.children[i].delete(key);
            } else {
                // b
                if i >= self.key_len() {
                    i -= 1;
                }
                self.unionchild(i);
                return self.delete(key);
            }
        }
        self.children[i].delete(key)
    }

    // 调用条件：self.children[i]只有t-1个关键字，但是他的右兄弟self.children[i + 1]存在且至少有t个关键字
    // 待删除节点存在于子树self.children[i]中，但是self.children[i]只有t-1个关键字key，直接删除就不满足B树的结构
    // 此时self.children[i + 1]至少有t个关键字，则将self的一个关键字降至self.children[i]中，
    // 再将self.children[i + 1]的一个关键字升至self，最后将self.children[i + 1]相应的孩子移到self.children[i]
    pub fn roleft(&mut self, i: usize) {
        self.children[i].keys.push(self.keys[i].clone());
        if !self.children[i + 1].children.is_empty() {
            let new_child = std::mem::replace(
                &mut self.children[i + 1].children[0],
                Node::new(self.degree, None, None),
            );
            self.children[i].children.push(new_child);
            self.children[i + 1].children.remove(0);
        }

        self.keys[i] = self.children[i + 1].keys[0].clone();
        self.children[i + 1].keys.remove(0);
    }

    // 调用条件：self.children[i]只有t-1个关键字，但是他的左兄弟self.children[i - 1]存在且至少有t个关键字
    // 待删除节点存在于子树self.children[i]中，但是self.children[i]只有t-1个关键字key，直接删除就不满足B树的结构
    // 此时self.children[i - 1]至少有t个关键字，则将self的某个关键字降至self.children[i]中，
    // 再将self.children[i - 1]的某个关键字升至self，最后将self.children[i - 1]相应的孩子移到self.children[i]
    pub fn roright(&mut self, i: usize) {
        self.children[i].keys.insert(0, self.keys[i - 1].clone());
        if !self.children[i - 1].children.is_empty() {
            let new_child = self.children[i - 1].children.pop().unwrap();
            self.children[i].children.insert(0, new_child);
        }

        self.keys[i - 1] = self.children[i - 1].keys.pop().unwrap();
    }

    // 设当前节点的index指向的key关键字记为k, 当前节点前于k的子节点记为y，当前节点后于k的子节点记为z。
    // 该函数将k和z全部合并进y。这样当前节点就失去了k和指向z的指针。
    // 输入必须保证：y和z必须存在且y和z的关键字个数都等于t - 1
    // 调用完成后y的关键字个数为2t-1
    pub fn unionchild(&mut self, i: usize) {
        let mut right_child = std::mem::replace(
            &mut self.children[i + 1],
            Node::new(self.degree, None, None),
        );
        //let mut right_child = &mut self.children[index + 1];
        let left_child = &mut self.children[i];

        left_child.keys.push(self.keys[i].clone());
        left_child.keys.append(&mut right_child.keys);

        left_child.children.append(&mut right_child.children);
        self.keys.remove(i);
        self.children.remove(i + 1);
        if self.key_len() == 0 {
            *self = std::mem::replace(&mut self.children[i], Node::new(self.degree, None, None));
        }
    }

    pub fn search(&self, key: &T) -> Option<(&Node<T>, usize)> {
        let mut i = 0;
        while i < self.key_len() && *key > self.keys[i] {
            i += 1;
        }
        if i < self.key_len() && *key == self.keys[i] {
            Some((self, i))
        } else if self.is_leaf() {
            None
        } else {
            self.children[i].search(key)
        }
    }

    pub fn in_order(&self, buf: &mut Vec<T>) {
        if self.is_leaf() {
            for x in &self.keys {
                buf.push(x.clone());
            }
            return;
        }
        for i in 0..self.key_len() {
            self.children[i].in_order(buf);
            buf.push(self.keys[i].clone());
        }
        self.children[self.key_len()].in_order(buf);
    }

    pub fn level_order(&self, buf: &mut Vec<T>) {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                for k in &node.keys {
                    buf.push(k.clone());
                }
                for c in &node.children {
                    queue.push_back(c);
                }
            }
        }
    }
}
