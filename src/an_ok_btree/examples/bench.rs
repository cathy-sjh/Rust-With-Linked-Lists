use an_ok_btree::BTree;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut tree = BTree::new(1024);
    for i in 0..10000 {
        tree.insert(i);
    }
    let elapsed_time = now.elapsed();
    println!(
        "B Tree insert 10000 times took {} ms.",
        elapsed_time.as_millis()
    );
}