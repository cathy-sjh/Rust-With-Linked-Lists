use an_unsafe_rb_tree::RBTree;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut tree = RBTree::new();
    for i in 0..10000 {
        tree.insert(i, i);
    }
    let elapsed_time = now.elapsed();
    println!(
        "RB Tree insert 10000 times took {} ms.",
        elapsed_time.as_millis()
    );
}
