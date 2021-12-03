#[cfg(test)]
mod tests {
    use an_ok_nonnull_deque::List;

    #[test]
    fn front_basics() {
        let mut list = List::new();
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.pop_front(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn back_basics() {
        let mut list = List::new();
        assert_eq!(list.peek_back(), None);
        assert_eq!(list.pop_back(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.peek_back(), Some(&1));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn front_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek_back(), None);
        assert_eq!(list.peek_front(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.peek_back(), Some(&3));
        assert_eq!(list.peek_front(), Some(&1));
        list.pop_front();
        list.pop_front();
        list.pop_front();
        assert_eq!(list.peek_back(), None);
        assert_eq!(list.peek_front(), None);
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert_eq!(list.peek_back_mut(), None);
        assert_eq!(list.peek_front_mut(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.peek_back_mut(), Some(&mut 3));
        list.peek_back_mut().map(|value| {
            *value = 11;
        });
        assert_eq!(list.peek_back_mut(), Some(&mut 11));
        assert_eq!(list.peek_front_mut(), Some(&mut 1));

        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.peek_front_mut(), Some(&mut 3));
        list.peek_front_mut().map(|value| {
            *value = 11;
        });
        assert_eq!(list.peek_front_mut(), Some(&mut 11));
        assert_eq!(list.peek_back_mut(), Some(&mut 1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter_mut();
        iter.next().map(|elem| *elem = 4);
        iter.next().map(|elem| *elem = 5);

        iter.next().map(|elem| *elem = 6);
        for elem in list.iter_mut() {
            *elem += 1;
        }
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn iter_mut_rev() {
        let mut list = List::new();
        list.push_back("apple".to_string());
        list.push_back("banana".to_string());
        list.push_back("sky".to_string());
        let mut iter = list.iter_mut();

        iter.next_back().map(|node| {
            node.push_str(" blue");
        });
        iter.next_back().map(|node| {
            node.push_str(" yello");
        });
        iter.next_back().map(|node| {
            node.push_str(" red");
        });

        for name in list.iter_mut().rev() {
            name.push_str(" yes");
        }
        assert_eq!(list.pop_back(), Some(String::from("sky blue yes")));
        assert_eq!(list.pop_back(), Some(String::from("banana yello yes")));
        assert_eq!(list.pop_back(), Some(String::from("apple red yes")));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn append() {
        let mut list1 = List::new();
        list1.push_back('a');

        let mut list2 = List::new();
        list2.push_back('b');
        list2.push_back('c');

        list1.append(&mut list2);

        let mut iter = list1.iter();
        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.next(), Some(&'c'));
        assert!(iter.next().is_none());
        assert_eq!(list2.peek_front(), None);
    }

    #[test]
    fn clear() {
        let mut list = List::new();
        assert!(list.is_empty());
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert!(!list.is_empty());
        list.clear();
        assert!(list.is_empty());
    }
}
