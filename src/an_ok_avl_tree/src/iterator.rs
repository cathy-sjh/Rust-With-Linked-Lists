use crate::AVLTree;
use crate::Queue;
use std::collections::Bound;

// 范围迭代器
pub struct RangePairIter<'a, K: PartialOrd + Clone, V> {
    tree: &'a AVLTree<K, V>,
    from: Bound<K>,
    to: Bound<K>,
    prev: Option<&'a K>,
}

impl<'a, K: PartialOrd + Clone, V> RangePairIter<'a, K, V> {
    pub fn new(tree: &'a AVLTree<K, V>, lower: Bound<K>, upper: Bound<K>) -> Self {
        Self {
            tree,
            from: lower,
            to: upper,
            prev: None,
        }
    }

    fn get_next_key_under(&mut self) -> Option<(&'a K, &'a V)> {
        let res = self
            .get_next_pair()
            .and_then(|cur| self.check_upper_bound(cur));
        if let Some((key, _)) = res {
            self.prev = Some(key);
        }
        res
    }

    fn get_next_pair(&mut self) -> Option<(&'a K, &'a V)> {
        match self.prev {
            None => self.get_lower_bound_pair(),
            Some(key) => self.tree.successor(key),
        }
    }

    fn get_lower_bound_pair(&self) -> Option<(&'a K, &'a V)> {
        match self.from {
            Bound::Included(ref key) => {
                self.tree.get_pair(key).or_else(|| self.tree.successor(key))
            }
            Bound::Excluded(ref key) => self.tree.successor(key),
            Bound::Unbounded => self.tree.min_pair(),
        }
    }

    fn check_upper_bound(&self, current: (&'a K, &'a V)) -> Option<(&'a K, &'a V)> {
        let ok = match self.to {
            Bound::Included(ref key) => current.0 <= key,
            Bound::Excluded(ref key) => current.0 < key,
            Bound::Unbounded => true,
        };
        if ok {
            Some(current)
        } else {
            None
        }
    }
}

impl<'a, K: PartialOrd + Clone, V> Iterator for RangePairIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_key_under()
    }
}

//遍历迭代器
pub struct TraverseIter<'a, K, V> {
    data: Queue<(&'a K, &'a V)>,
}

impl<'a, K, V> TraverseIter<'a, K, V> {
    pub fn new(queue: Queue<(&'a K, &'a V)>) -> Self {
        TraverseIter { data: queue }
    }
}

impl<'a, K: PartialOrd + Clone, V> Iterator for TraverseIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}
