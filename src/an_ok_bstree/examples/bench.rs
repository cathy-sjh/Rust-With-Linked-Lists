use std::time::Instant;
use an_ok_bstree::BSTree;

fn main() {
    let now  = Instant::now();
    let mut tree = BSTree::new();
    for i in 0..10000 {
        tree.insert(i, i);
    }
    let elapsed_time = now.elapsed();
    #[cfg(feature = "no_recur")]
    println!("No Recursion: BSTree insert 10000 times took {} ms.", elapsed_time.as_millis());
    #[cfg(not(feature = "no_recur"))]
    println!("Recursion: BSTree insert 10000 times took {} ms.", elapsed_time.as_millis());
}