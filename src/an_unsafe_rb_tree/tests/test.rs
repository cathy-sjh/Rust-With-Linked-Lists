#[cfg(test)]
mod tests {
    use an_unsafe_rb_tree::RBTree;
    #[test]
    fn insert_delete() {
        let mut tree = RBTree::new();
        assert!(tree.is_empty());
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        tree.insert(5, 'e');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(10, 'j');
        tree.insert(9, 'i');
        tree.insert(8, 'h');
        tree.delete(6);
        tree.delete(4);
        assert_eq!(tree.to_string(), "[K: 5, V: e, C: Black L: [K: 2, V: b, C: Black L: [K: 1, V: a, C: Black L: Ø, R: Ø]\
        , R: [K: 3, V: c, C: Black L: Ø, R: Ø]], R: [K: 9, V: i, C: Black L: [K: 7, V: g, C: Black L: Ø, R: [K: 8, V: h, C: Red L: Ø, R: Ø]]\
        , R: [K: 10, V: j, C: Black L: Ø, R: Ø]]]".to_string());
        /*
                         5
                       /   \
                     2       9
                    / \     /  \
                   1   3   7     10
                            \
                             8(r)
        */
        assert_eq!(tree.tree_height(), 4);
        assert!(!tree.contains(&4));
        assert!(!tree.contains(&6));
        tree.delete(8);
        assert_eq!(tree.to_string(), "[K: 5, V: e, C: Black L: [K: 2, V: b, C: Black L: [K: 1, V: a, C: Black L: Ø, R: Ø]\
        , R: [K: 3, V: c, C: Black L: Ø, R: Ø]], R: [K: 9, V: i, C: Black L: [K: 7, V: g, C: Black L: Ø, R: Ø]\
        , R: [K: 10, V: j, C: Black L: Ø, R: Ø]]]".to_string());
        /*
                         5
                       /   \
                     2       9
                    / \     /  \
                   1   3   7     10
        */
        assert_eq!(tree.tree_height(), 3);
        tree.delete(9);
        assert_eq!(tree.to_string(), "[K: 5, V: e, C: Black L: [K: 2, V: b, C: Red L: [K: 1, V: a, C: Black L: Ø, R: Ø]\
        , R: [K: 3, V: c, C: Black L: Ø, R: Ø]], R: [K: 10, V: j, C: Black L: [K: 7, V: g, C: Red L: Ø, R: Ø], R: Ø]]".to_string());
        /*
                         5
                       /   \
                    2(r)    10
                    / \     /
                   1   3  7(r)
        */
        assert_eq!(tree.tree_height(), 3);
        tree.delete(10);
        assert_eq!(
            tree.to_string(),
            "[K: 5, V: e, C: Black L: [K: 2, V: b, C: Red L: [K: 1, V: a, C: Black L: Ø, R: Ø]\
        , R: [K: 3, V: c, C: Black L: Ø, R: Ø]], R: [K: 7, V: g, C: Black L: Ø, R: Ø]]"
                .to_string()
        );
        /*
                     5
                   /   \
                2(r)    7
                / \
               1   3
        */
        assert_eq!(tree.tree_height(), 3);
        tree.delete(7);
        assert_eq!(
            tree.to_string(),
            "[K: 2, V: b, C: Black L: [K: 1, V: a, C: Black L: Ø, R: Ø]\
        , R: [K: 5, V: e, C: Black L: [K: 3, V: c, C: Red L: Ø, R: Ø], R: Ø]]"
                .to_string()
        );
        /*
                     2
                   /   \
                 1      5
                       /
                      3(r)
        */
        assert_eq!(tree.tree_height(), 3);
        tree.delete(2);
        assert_eq!(tree.to_string(), "[K: 3, V: c, C: Black L: [K: 1, V: a, C: Black L: Ø, R: Ø], R: [K: 5, V: e, C: Black L: Ø, R: Ø]]".to_string());
        /*
                 3
               /   \
             1      5
        */
        assert_eq!(tree.tree_height(), 2);
        tree.delete(3);
        assert_eq!(
            tree.to_string(),
            "[K: 5, V: e, C: Black L: [K: 1, V: a, C: Red L: Ø, R: Ø], R: Ø]".to_string()
        );
        /*
                 5
               /
             1(r)
        */
        assert_eq!(tree.tree_height(), 2);
        tree.delete(5);
        assert_eq!(
            tree.to_string(),
            "[K: 1, V: a, C: Black L: Ø, R: Ø]".to_string()
        );
        /*
                 1
        */
        assert_eq!(tree.tree_height(), 1);
        tree.delete(1);
        assert_eq!(tree.to_string(), "Ø".to_string());
        assert!(tree.is_empty());
    }

    /*
                     4
                   /   \
                 2       6
                / \     /  \
               1   3   5     9(r)
                           /   \
                          7    10
                           \
                            8(r)
    */
    #[test]
    fn height() {
        let mut tree: RBTree<i32, char> = RBTree::new();
        assert!(tree.is_empty());
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        tree.insert(5, 'e');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(10, 'j');
        tree.insert(9, 'i');
        tree.insert(8, 'h');
        assert_eq!(tree.tree_height(), 5);
    }

    #[test]
    fn max_min_get_pair() {
        let mut tree = RBTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        tree.insert(5, 'e');
        tree.insert(6, 'f');
        tree.insert(7, 'g');
        tree.insert(10, 'j');
        tree.insert(9, 'i');
        tree.insert(8, 'h');
        assert_eq!(tree.get(&4), Some(&'d'));
        assert_eq!(tree.get(&2), Some(&'b'));
        assert_eq!(tree.get(&9), Some(&'i'));
        assert_eq!(tree.get(&10), Some(&'j'));
        assert_eq!(tree.get_pair(&4), Some((&4, &'d')));
        assert_eq!(tree.get_pair(&9), Some((&9, &'i')));
        assert_eq!(tree.get_pair(&11), None);
        assert_eq!(tree.min_pair(), Some((&1, &'a')));
        assert_eq!(tree.max_pair(), Some((&10, &'j')));
        assert!(tree.contains(&10));
        assert!(!tree.contains(&12));
    }

    #[test]
    fn successor_predecessor() {
        let mut tree = RBTree::new();
        tree.insert(3, "3");
        tree.insert(2, "2");
        tree.insert(1, "1");
        tree.insert(4, "4");
        tree.insert(5, "5");
        tree.insert(6, "6");
        tree.insert(7, "7");
        tree.insert(10, "10");
        tree.insert(9, "9");
        tree.insert(8, "8");
        assert_eq!(tree.successor(&6), Some((&7, &"7")));
        assert_eq!(tree.successor(&3), Some((&4, &"4")));
        assert_eq!(tree.predecessor(&5), Some((&4, &"4")));
        assert_eq!(tree.successor(&10), None);
        assert_eq!(tree.predecessor(&1), None);
        assert_eq!(tree.successor(&0), None);
        assert_eq!(tree.predecessor(&100), None);
    }

    #[test]
    fn test_traverse_iter() {
        let mut tree = RBTree::new();
        tree.insert(3, "3");
        tree.insert(2, "2");
        tree.insert(1, "1");
        tree.insert(4, "4");
        tree.insert(5, "5");
        tree.insert(6, "6");
        tree.insert(7, "7");
        tree.insert(10, "10");
        tree.insert(9, "9");
        tree.insert(8, "8");
        let res: Vec<(&i32, &&str)> = tree.preorder_iter().collect();
        assert_eq!(
            res,
            vec![
                (&4, &"4"),
                (&2, &"2"),
                (&1, &"1"),
                (&3, &"3"),
                (&6, &"6"),
                (&5, &"5"),
                (&9, &"9"),
                (&7, &"7"),
                (&8, &"8"),
                (&10, &"10")
            ]
        );
        let res: Vec<(&i32, &&str)> = tree.inorder_iter().collect();
        assert_eq!(
            res,
            vec![
                (&1, &"1"),
                (&2, &"2"),
                (&3, &"3"),
                (&4, &"4"),
                (&5, &"5"),
                (&6, &"6"),
                (&7, &"7"),
                (&8, &"8"),
                (&9, &"9"),
                (&10, &"10")
            ]
        );
        let res: Vec<(&i32, &&str)> = tree.postorder_iter().collect();
        assert_eq!(
            res,
            vec![
                (&1, &"1"),
                (&3, &"3"),
                (&2, &"2"),
                (&5, &"5"),
                (&8, &"8"),
                (&7, &"7"),
                (&10, &"10"),
                (&9, &"9"),
                (&6, &"6"),
                (&4, &"4")
            ]
        );
        let res: Vec<(&i32, &&str)> = tree.levelorder_iter().collect();
        assert_eq!(
            res,
            vec![
                (&4, &"4"),
                (&2, &"2"),
                (&6, &"6"),
                (&1, &"1"),
                (&3, &"3"),
                (&5, &"5"),
                (&9, &"9"),
                (&7, &"7"),
                (&10, &"10"),
                (&8, &"8")
            ]
        );
    }

    #[test]
    fn to_string() {
        let mut tree = RBTree::new();
        tree.insert(3, 'c');
        tree.insert(2, 'b');
        tree.insert(1, 'a');
        tree.insert(4, 'd');
        assert_eq!(tree.to_string(), String::from("[K: 2, V: b, C: Black L: [K: 1, V: a, C: Black L: Ø, R: Ø], R: [K: 3, V: c, C: Black L: Ø, R: [K: 4, V: d, C: Red L: Ø, R: Ø]]]"))
    }
}
