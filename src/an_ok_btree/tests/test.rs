#[cfg(test)]
mod tests {
    use an_ok_btree::BTree;
    /*
                     6
                   /   \
                 2     8,10
               /  \    / / \
              1 3,4,5 7 9  11,12
    */
    #[test]
    fn insert_delete() {
        let mut tree = BTree::new(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(9);
        tree.insert(8);
        tree.insert(4);
        tree.insert(3);
        tree.insert(12);
        tree.insert(10);
        tree.insert(11);

        tree.delete(1);
        assert!(!tree.contain(&1));
        /*
                         8
                       /   \
                    3,6     10
                   / | \    / \
                  2 4,5 7  9  11,12
        */
        assert_eq!(
            tree.to_string(),
            "BNode([8], [BNode([3, 6], [BNode([2], []), BNode([4, 5], []), BNode([7], [])])\
                                    , BNode([10], [BNode([9], []), BNode([11, 12], [])])])"
                .to_string()
        );

        tree.delete(2);
        assert!(!tree.contain(&2));
        /*
                         8
                       /   \
                    4,6     10
                   / | \    / \
                  3  5  7  9  11,12
        */
        assert_eq!(
            tree.to_string(),
            "BNode([8], [BNode([4, 6], [BNode([3], []), BNode([5], []), BNode([7], [])])\
                                        , BNode([10], [BNode([9], []), BNode([11, 12], [])])])"
                .to_string()
        );

        tree.delete(8);
        assert!(!tree.contain(&8));
        /*
                         7
                       /   \
                     4     10
                   /  \    / \
                  3   5,6 9  11,12
        */
        assert_eq!(
            tree.to_string(),
            "BNode([7], [BNode([4], [BNode([3], []), BNode([5, 6], [])])\
                                    , BNode([10], [BNode([9], []), BNode([11, 12], [])])])"
                .to_string()
        );

        tree.delete(7);
        assert!(!tree.contain(&7));
        /*
                     4, 6, 10
                   /  /   /  \
                  3  5   9  11,12
        */
        assert_eq!(tree.to_string(), "BNode([4, 6, 10], [BNode([3], []), BNode([5], []), BNode([9], []), BNode([11, 12], [])])".to_string());

        tree.delete(10);
        assert!(!tree.contain(&10));
        /*
                     4, 6, 11
                   /  /   /  \
                  3  5   9    12
        */
        assert_eq!(
            tree.to_string(),
            "BNode([4, 6, 11], [BNode([3], []), BNode([5], []), BNode([9], []), BNode([12], [])])"
                .to_string()
        );

        // 删除一个不存在的节点，也会引起树的变化
        tree.delete(30);
        assert!(!tree.contain(&30));
        /*
                      4,  6
                     /   /  \
                   3    5   9,11,12
        */
        assert_eq!(
            tree.to_string(),
            "BNode([4, 6], [BNode([3], []), BNode([5], []), BNode([9, 11, 12], [])])".to_string()
        );

        tree.delete(3);
        assert!(!tree.contain(&3));
        /*
                        6
                     /     \
                   4,5    9,11,12
        */
        assert_eq!(
            tree.to_string(),
            "BNode([6], [BNode([4, 5], []), BNode([9, 11, 12], [])])".to_string()
        );

        tree.delete(12);
        assert!(!tree.contain(&12));
        /*
                        6
                     /     \
                   4,5    9,11
        */
        assert_eq!(
            tree.to_string(),
            "BNode([6], [BNode([4, 5], []), BNode([9, 11], [])])".to_string()
        );

        tree.delete(9);
        assert!(!tree.contain(&9));
        /*
                        6
                     /     \
                   4,5      11
        */
        assert_eq!(
            tree.to_string(),
            "BNode([6], [BNode([4, 5], []), BNode([11], [])])".to_string()
        );

        tree.delete(11);
        assert!(!tree.contain(&11));
        /*
                        5
                      /   \
                     4     6
        */
        assert_eq!(
            tree.to_string(),
            "BNode([5], [BNode([4], []), BNode([6], [])])".to_string()
        );

        tree.delete(6);
        assert!(!tree.contain(&6));
        /*
                        4,5
        */
        assert_eq!(tree.to_string(), "BNode([4, 5], [])".to_string());

        tree.delete(5);
        assert!(!tree.contain(&5));
        /*
                        4
        */
        assert_eq!(tree.to_string(), "BNode([4], [])".to_string());

        tree.delete(4);
        assert!(!tree.contain(&4));
        assert_eq!(tree.to_string(), "BNode([], [])".to_string());
        assert!(tree.is_empty());

        tree.delete(4);
        tree.delete(4);
        assert!(tree.is_empty());
    }

    #[test]
    fn double_insert() {
        let mut tree = BTree::new(2);
        for _ in 0..6 {
            tree.insert(0);
        }
        assert_eq!(tree.to_string(), "BNode([0, 0], [BNode([0, 0], []), BNode([0], []), BNode([0], [])])".to_string());
    }

    #[test]
    fn test_empty() {
        let mut t = BTree::new(2);
        assert!(t.is_empty());
        t.insert(1);
        t.insert(3);
        assert!(!t.is_empty());
    }

    #[test]
    fn degree() {
        let mut tree =  BTree::new(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(9);
        tree.insert(8);
        tree.insert(4);
        tree.insert(3);
        tree.insert(12);
        tree.insert(10);
        tree.insert(11);
        tree.insert(5);
        tree.insert(5);
        tree.insert(5);
        assert_eq!(tree.to_string(), "BNode([3, 5, 8], [BNode([1, 2], []), BNode([4, 5, 5, 5], []), BNode([6, 7], []), BNode([9, 10, 11, 12], [])])".to_string());
        tree.delete(10);
        tree.delete(5);
        tree.delete(11);
        tree.delete(1);
        assert_eq!(tree.to_string(), "BNode([4, 5, 8], [BNode([2, 3], []), BNode([5, 5], []), BNode([6, 7], []), BNode([9, 12], [])])".to_string());
    }

    #[test]
    fn min_max_contain() {
        let mut tree = BTree::new(2);
        assert_eq!(tree.find_max(), None);
        assert_eq!(tree.find_min(), None);
        tree.insert(7);
        tree.insert(1);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(9);
        tree.insert(8);
        tree.insert(4);
        tree.insert(3);
        tree.insert(12);
        tree.insert(10);
        tree.insert(11);
        assert!(tree.contain(&1));
        assert!(tree.contain(&2));
        assert!(tree.contain(&3));
        assert!(tree.contain(&4));
        assert!(tree.contain(&5));
        assert!(tree.contain(&6));
        assert!(tree.contain(&7));
        assert!(tree.contain(&8));
        assert!(tree.contain(&9));
        assert!(tree.contain(&10));
        assert!(tree.contain(&11));
        assert!(tree.contain(&12));
        assert!(!tree.contain(&13));
        assert!(!tree.contain(&14));

        assert_eq!(tree.find_max(), Some(12));
        assert_eq!(tree.find_min(), Some(1));
    }

    #[test]
    fn successor_predecessor() {
        let mut tree = BTree::new(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(9);
        tree.insert(8);
        tree.insert(4);
        tree.insert(3);
        tree.insert(12);
        tree.insert(10);
        tree.insert(11);
        assert_eq!(tree.successor(0), None);
        assert_eq!(tree.successor(1), Some(2));
        assert_eq!(tree.successor(2), Some(3));
        assert_eq!(tree.successor(3), Some(4));
        assert_eq!(tree.successor(4), Some(5));
        assert_eq!(tree.successor(5), Some(6));
        assert_eq!(tree.successor(6), Some(7));
        assert_eq!(tree.successor(7), Some(8));
        assert_eq!(tree.successor(8), Some(9));
        assert_eq!(tree.successor(9), Some(10));
        assert_eq!(tree.successor(10), Some(11));
        assert_eq!(tree.successor(11), Some(12));
        assert_eq!(tree.successor(12), None);

        assert_eq!(tree.predecessor(0), None);
        assert_eq!(tree.predecessor(1), None);
        assert_eq!(tree.predecessor(2), Some(1));
        assert_eq!(tree.predecessor(3), Some(2));
        assert_eq!(tree.predecessor(4), Some(3));
        assert_eq!(tree.predecessor(5), Some(4));
        assert_eq!(tree.predecessor(6), Some(5));
        assert_eq!(tree.predecessor(7), Some(6));
        assert_eq!(tree.predecessor(8), Some(7));
        assert_eq!(tree.predecessor(9), Some(8));
        assert_eq!(tree.predecessor(10), Some(9));
        assert_eq!(tree.predecessor(11), Some(10));
        assert_eq!(tree.predecessor(12), Some(11));
        assert_eq!(tree.predecessor(13), None);
    }

    #[test]
    fn test_traverse_iter() {
        let mut tree = BTree::new(2);
        let res: Vec<i32> = tree.inorder_iter().collect();
        assert!(res.is_empty());
        let res: Vec<i32> = tree.levelorder_iter().collect();
        assert!(res.is_empty());
        tree.insert(7);
        tree.insert(1);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(9);
        tree.insert(8);
        tree.insert(4);
        tree.insert(3);
        tree.insert(12);
        tree.insert(10);
        tree.insert(11);
        let res: Vec<i32> = tree.inorder_iter().collect();
        assert_eq!(res, vec![1,2,3,4,5,6,7,8,9,10,11,12]);
        let res: Vec<i32> = tree.levelorder_iter().collect();
        assert_eq!(res, vec![6,2,8,10,1,3,4,5,7,9,11,12]);
    }
}
