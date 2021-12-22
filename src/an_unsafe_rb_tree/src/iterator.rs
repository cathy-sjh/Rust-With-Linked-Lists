use std::collections::VecDeque;

//遍历迭代器，包括前序、中序、后序、层序
pub struct TraverseIter<'a, K, V> {
    data: VecDeque<(&'a K, &'a V)>,
}

impl<'a, K, V> TraverseIter<'a, K, V> {
    pub fn new(queue: VecDeque<(&'a K, &'a V)>) -> Self {
        TraverseIter { data: queue }
    }
}

impl<'a, K: PartialOrd + Clone, V> Iterator for TraverseIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}
