#[cfg(test)]
mod tests {
    use an_ok_bstree::BSTree;

    /*
                10
               / \
             5    15
            / \   / \
          3   7  14  17
         / \
        2  4

    */

    #[test]
    fn insert_delete() {
        let mut tree = BSTree::new();
        tree.insert(8, 'h');
        tree.insert(5, 'e');
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(4, 'd');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(15, 'o');
        tree.insert(12, 'l');
        tree.insert(17, 'q');
        tree.insert(10, 'j');
        tree.insert(14, 'n');
        tree.delete(12);
        tree.delete(6);
        tree.delete(8);
        assert!(tree.contains(&5));
        assert!(!tree.contains(&12));
        assert!(!tree.contains(&6));
        assert!(!tree.contains(&8));
        assert_eq!(tree.successor(&4), Some((&5, &'e')));
        assert_eq!(tree.successor(&5), Some((&7, &'g')));
        assert_eq!(tree.successor(&10), Some((&14, &'n')));
        let res: Vec<(&i32, &char)> = tree.preorder_iter().collect();
        assert_eq!(res, vec![(&10, &'j'), (&5, &'e'), (&3, &'c'), (&2, &'b'), (&4, &'d'), (&7, &'g')
                            ,(&15, &'o'), (&14, &'n'), (&17, &'q')]);
    }

    #[test]
    fn delete_tree() {
        /*
                8
               / \
             5    15
            / \   / \
          3   6  12  17
         / \   \  / \
        2  4   7 10 14

    */
        let mut tree = BSTree::new();
        tree.insert(8, 'h');
        tree.insert(5, 'e');
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(4, 'd');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(15, 'o');
        tree.insert(12, 'l');
        tree.insert(17, 'q');
        tree.insert(10, 'j');
        tree.insert(14, 'n');
        tree.delete_tree(2);
        assert!(!tree.contains(&2));
        tree.delete_tree(5);
        let res: Vec<(&i32, &char)> = tree.inorder_iter().collect();
        assert_eq!(res, vec![(&8, &'h'), (&10, &'j'), (&12, &'l'), (&14, &'n'), (&15, &'o'), (&17, &'q')]);
        tree.delete_tree(8);
        assert!(tree.is_empty());
    }

    #[test]
    fn remove_tree() {
        /*
                8
               / \
             5    15
            / \   / \
          3   6  12  17
         / \   \  / \
        2  4   7 10 14

    */
        let mut tree = BSTree::new();
        tree.insert(8, 'h');
        tree.insert(5, 'e');
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(4, 'd');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(15, 'o');
        tree.insert(12, 'l');
        tree.insert(17, 'q');
        tree.insert(10, 'j');
        tree.insert(14, 'n');

        assert!(tree.remove_tree(100).is_empty());
        let rm_tree = tree.remove_tree(5);

        let res: Vec<(&i32, &char)> = tree.inorder_iter().collect();
        assert_eq!(res, vec![(&8, &'h'), (&10, &'j'), (&12, &'l'), (&14, &'n'), (&15, &'o'), (&17, &'q')]);
        let res: Vec<(&i32, &char)> = rm_tree.inorder_iter().collect();
        assert_eq!(res, vec![(&2, &'b'), (&3, &'c'), (&4, &'d'), (&5, &'e'), (&6, &'f'), (&7, &'g')]);

        let rm_tree = tree.remove_tree(8);
        assert!(tree.is_empty());
        let res: Vec<(&i32, &char)> = rm_tree.inorder_iter().collect();
        assert_eq!(res, vec![(&8, &'h'), (&10, &'j'), (&12, &'l'), (&14, &'n'), (&15, &'o'), (&17, &'q')]);
    }

    #[test]
    fn test_empty() {
        let data = 1337;
        let mut t = BSTree::new();
        assert!(t.is_empty());
        t.insert(1, data);
        assert!(!t.is_empty());
    }

    #[test]
    fn max_min_get_pair() {
        /*
             8
            / \
          5    15
         / \   / \
       3   6  12  17
      / \   \  / \
     2  4   7 10 14

 */
        let mut tree = BSTree::new();
        tree.insert(8, 'h');
        tree.insert(5, 'e');
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(4, 'd');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(15, 'o');
        tree.insert(12, 'l');
        tree.insert(17, 'q');
        tree.insert(10, 'j');
        tree.insert(14, 'n');
        assert_eq!(tree.min_pair(), Some((&2, &'b')));
        assert_eq!(tree.max_pair(), Some((&17, &'q')));
        assert_eq!(tree.get_pair(&4), Some((&4, &'d')));
        tree.insert(4, 'y');
        assert_eq!(tree.get_pair(&4), Some((&4, &'y')));
        assert_eq!(tree.get_pair(&12), Some((&12, &'l')));
        assert_eq!(tree.get_pair(&11), None);
        assert_eq!(tree.get(&4), Some(&'y'));
        assert_eq!(tree.get(&12), Some(&'l'));
        assert_eq!(tree.get(&15), Some(&'o'));
        assert_eq!(tree.get(&19), None);
        assert_eq!(tree.get_or(&5, &'z'), &'e');
        assert_eq!(tree.get_or(&6, &'z'), &'f');
        assert_eq!(tree.get_or(&11, &'z'), &'z');
        assert!(tree.contains(&10));
        assert!(!tree.contains(&22));
    }

    #[test]
    fn successor_predecessor() {
        /*
                8
               / \
             5    15
            / \   / \
          3   6  12  17
         / \   \  / \
        2  4   7 10 14

    */
        let mut tree = BSTree::new();
        tree.insert(8, 'h');
        tree.insert(5, 'e');
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(4, 'd');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(15, 'o');
        tree.insert(12, 'l');
        tree.insert(17, 'q');
        tree.insert(10, 'j');
        tree.insert(14, 'n');
        assert_eq!(tree.successor(&6), Some((&7, &'g')));
        assert_eq!(tree.successor(&3), Some((&4, &'d')));
        assert_eq!(tree.predecessor(&5), Some((&4, &'d')));
        assert_eq!(tree.successor(&17), None);
        assert_eq!(tree.predecessor(&1), None);
        assert_eq!(tree.successor(&0), Some((&2, &'b')));
        assert_eq!(tree.predecessor(&100), Some((&17, &'q')));
    }

    #[test]
    fn test_traverse_iter() {
        let mut tree = BSTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        let res: Vec<(&i32, &char)> = tree.preorder_iter().collect();
        assert_eq!(res, vec![(&3, &'c'), (&2, &'b'), (&1, &'a'), (&4, &'d')]);
        let res: Vec<(&i32, &char)> = tree.inorder_iter().collect();
        assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&3, &'c'), (&4, &'d')]);
        let res: Vec<(&i32, &char)> = tree.postorder_iter().collect();
        assert_eq!(res, vec![(&1, &'a'), (&2, &'b'), (&4, &'d'), (&3, &'c')]);
        let res: Vec<(&i32, &char)> = tree.levelorder_iter().collect();
        assert_eq!(res, vec![(&3, &'c'), (&2, &'b'), (&4, &'d'), (&1, &'a')]);
    }
}