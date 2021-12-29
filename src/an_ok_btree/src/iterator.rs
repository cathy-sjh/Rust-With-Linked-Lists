use std::collections::VecDeque;

//遍历迭代器
pub struct TraverseIter<T> {
    data: VecDeque<T>,
}

impl<T> TraverseIter<T> {
    pub fn new(queue: VecDeque<T>) -> Self {
        TraverseIter { data: queue }
    }
}

impl<T> Iterator for TraverseIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}