#[cfg(test)]
mod tests {
    use an_ok_bstree::BSTree;

    /*
                8
               / \
             5    15
            / \   / \
          3   6  12  7
         / \  /  / \
        2  4 7  10 14

    */

    #[test]
    fn basic() {
        let mut tree = BSTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);
        assert_eq!(tree.tree_height(), 4);
        assert_eq!(tree.tree_node_size(), 12);
        assert_eq!(tree.tree_leaf_size(), 6);
    }

    #[test]
    fn search() {
        let mut tree = BSTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);
        assert_eq!(tree.search_max(), Some(&17));
        assert_eq!(tree.search_min(), Some(&2));
        assert!(tree.search(&8));
        assert!(tree.search(&5));
        assert!(tree.search(&15));
        assert!(tree.search(&6));
        assert!(tree.search(&17));
        assert!(tree.search(&4));
        assert!(tree.search(&14));
        assert!(!tree.search(&100));
        assert!(!tree.search(&1));
        assert!(!tree.search(&0));
    }

    #[test]
    fn delete() {
        let mut tree = BSTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);
        assert_eq!(tree.search(&8), true);
        assert_eq!(tree.delete(8), true);
        assert_eq!(tree.search(&8), false);
        assert_eq!(tree.prev_orer(), vec![7, 5, 3, 2, 4, 6, 15, 12, 10, 14, 17]);
        assert_eq!(tree.in_order(), vec![2, 3, 4, 5, 6, 7, 10, 12, 14, 15, 17]);
        assert_eq!(tree.post_order(), vec![2, 4, 3, 6, 5, 10, 14, 12, 17, 15, 7]);
        assert_eq!(tree.level_order(), vec![7, 5, 15, 3, 6, 12, 17, 2, 4, 10, 14]);
    }

    #[test]
    fn delete_tree() {
        let mut tree = BSTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);
        assert!(tree.delete_tree(5));
        assert_eq!(tree.in_order(), vec![8, 10, 12, 14, 15, 17]);
        assert!(tree.delete_tree(8));
        assert!(tree.is_empty());
    }

    #[test]
    fn remove_tree() {
        let mut tree = BSTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);

        assert!(tree.remove_tree(100).is_empty());
        let rm_tree = tree.remove_tree(5);
        assert_eq!(tree.in_order(), vec![8, 10, 12, 14, 15, 17]);
        assert_eq!(rm_tree.in_order(), vec![2, 3, 4, 5, 6, 7]);
        let rm_tree = tree.remove_tree(8);
        assert!(tree.is_empty());
        assert_eq!(rm_tree.in_order(), vec![8, 10, 12, 14, 15, 17]);
    }

    #[test]
    fn iter() {
        let mut tree = BSTree::new();
        tree.insert(8);
        tree.insert(5);
        tree.insert(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(15);
        tree.insert(12);
        tree.insert(17);
        tree.insert(10);
        tree.insert(14);

        let res: Vec<i32> = tree.iter().collect();
        assert_eq!(res, vec![2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 15, 17]);
    }
}