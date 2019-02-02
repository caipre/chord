use std::collections::HashMap;

use proptest::*;
use proptest::prelude::*;

use super::*;

fn fixture_figure_3b() -> HashMap<usize, Node> {
    let mut m = HashMap::new();

    let node = Node::new(
        0,
        vec![
            Entry::new(1, 1),
            Entry::new(2, 3),
            Entry::new(4, 0),
        ],
    );
    m.insert(node.id, node);

    let node = Node::new(
        1,
        vec![
            Entry::new(2, 3),
            Entry::new(3, 3),
            Entry::new(5, 0),
        ],
    );
    m.insert(node.id, node);

    let node = Node::new(
        3,
        vec![
            Entry::new(4, 0),
            Entry::new(5, 0),
            Entry::new(7, 0),
        ],
    );
    m.insert(node.id, node);

    m
}

#[test]
fn test_figure_3b() {
    let m = fixture_figure_3b();
    let zer = m.get(&0).unwrap();
    let one = m.get(&1).unwrap();
    let thr = m.get(&3).unwrap();

    assert_eq!(zer.next_p(0), 3);
    assert_eq!(zer.next_p(1), 0);
    assert_eq!(zer.next_p(2), 1);
    assert_eq!(zer.next_p(3), 1);
    assert_eq!(zer.next_p(4), 3);
    assert_eq!(zer.next_p(5), 3);
    assert_eq!(zer.next_p(6), 3);
    assert_eq!(zer.next_p(7), 3);

    assert_eq!(one.next_p(0), 3);
    assert_eq!(one.next_p(1), 0);
    assert_eq!(one.next_p(2), 1);
    assert_eq!(one.next_p(3), 1);
    assert_eq!(one.next_p(4), 3);
    assert_eq!(one.next_p(5), 3);
    assert_eq!(one.next_p(6), 3);
    assert_eq!(one.next_p(7), 3);

    assert_eq!(thr.next_p(0), 3);
    assert_eq!(thr.next_p(1), 0);
    assert_eq!(thr.next_p(2), 0);
    assert_eq!(thr.next_p(3), 0);
    assert_eq!(thr.next_p(4), 3);
    assert_eq!(thr.next_p(5), 3);
    assert_eq!(thr.next_p(6), 3);
    assert_eq!(thr.next_p(7), 3);

    assert_eq!(zer.find_p(0, &m), 3);
    assert_eq!(zer.find_p(1, &m), 0);
    assert_eq!(zer.find_p(2, &m), 1);
    assert_eq!(zer.find_p(3, &m), 1);
    assert_eq!(zer.find_p(4, &m), 3);
    assert_eq!(zer.find_p(5, &m), 3);
    assert_eq!(zer.find_p(6, &m), 3);
    assert_eq!(zer.find_p(7, &m), 3);

    assert_eq!(one.find_p(0, &m), 3);
    assert_eq!(one.find_p(1, &m), 0);
    assert_eq!(one.find_p(2, &m), 1);
    assert_eq!(one.find_p(3, &m), 1);
    assert_eq!(one.find_p(4, &m), 3);
    assert_eq!(one.find_p(5, &m), 3);
    assert_eq!(one.find_p(6, &m), 3);
    assert_eq!(one.find_p(7, &m), 3);

    assert_eq!(thr.find_p(0, &m), 3);
    assert_eq!(thr.find_p(1, &m), 0);
    assert_eq!(thr.find_p(2, &m), 1);
    assert_eq!(thr.find_p(3, &m), 1);
    assert_eq!(thr.find_p(4, &m), 3);
    assert_eq!(thr.find_p(5, &m), 3);
    assert_eq!(thr.find_p(6, &m), 3);
    assert_eq!(thr.find_p(7, &m), 3);

    assert_eq!(zer.find_n(0, &m), 0);
    assert_eq!(zer.find_n(1, &m), 1);
    assert_eq!(zer.find_n(2, &m), 3);
    assert_eq!(zer.find_n(3, &m), 3);
    assert_eq!(zer.find_n(4, &m), 0);
    assert_eq!(zer.find_n(5, &m), 0);
    assert_eq!(zer.find_n(6, &m), 0);
    assert_eq!(zer.find_n(7, &m), 0);

    assert_eq!(one.find_n(0, &m), 0);
    assert_eq!(one.find_n(1, &m), 1);
    assert_eq!(one.find_n(2, &m), 3);
    assert_eq!(one.find_n(3, &m), 3);
    assert_eq!(one.find_n(4, &m), 0);
    assert_eq!(one.find_n(5, &m), 0);
    assert_eq!(one.find_n(6, &m), 0);
    assert_eq!(one.find_n(7, &m), 0);

    assert_eq!(thr.find_n(0, &m), 0);
    assert_eq!(thr.find_n(1, &m), 1);
    assert_eq!(thr.find_n(2, &m), 3);
    assert_eq!(thr.find_n(3, &m), 3);
    assert_eq!(thr.find_n(4, &m), 0);
    assert_eq!(thr.find_n(5, &m), 0);
    assert_eq!(thr.find_n(6, &m), 0);
    assert_eq!(thr.find_n(7, &m), 0);
}

#[test]
fn test_range_contains() {
    let modulo = 8;
    for start in 0..modulo {
        for end in (start + 1)..(start + modulo) {
            let endmod = end % modulo;
            let open = Range::open(start, endmod);
            let half = Range::half(start, endmod);
            assert!(!open.contains(start));
            assert!(!half.contains(start));
            for i in (start + 1)..end {
                let imod = i % modulo;
                assert!(open.contains(imod));
                assert!(half.contains(imod));
            }
            assert!(!open.contains(endmod));
            assert!(half.contains(endmod));
            for i in (end + 1)..(start + modulo) {
                let imod = i % modulo;
                assert!(!open.contains(imod));
                assert!(!half.contains(imod));
            }
        }
    }
}

proptest_compose! {
    fn arbitrary_in(start: usize, end: usize)
        (v in start..(end))
         -> usize {
    }
}

prop_compose! {
        fn arbitrary_range_open()
            (start in any::<usize>())
            (start in Just(start), end in start..std::usize::MAX)
            -> RangeOpen {
            Range::open(start, end)
        }
    }

prop_compose! {
        fn arbitrary_key_in_range_open(range: &RangeOpen)
            (key in range.range.start..range.range.end)
            -> usize {
            key
        }
    }

prop_compose! {
        fn arbitrary_key_and_range_open()
            (range in arbitrary_range_open())
            (key in arbitrary_key_in_range_open(&range), range in Just(range))
            -> (usize, RangeOpen) {
            (key, range)
        }
    }

prop_compose! {
        fn arbitrary_range_half()
            (start in any::<usize>())
            (start in Just(start), end in start..std::usize::MAX)
            -> RangeHalf {
            Range::half(start, end)
        }
    }

prop_compose! {
        fn arbitrary_key_in_range_half(range: &RangeHalf)
            (key in range.range.start..range.range.end)
            -> usize {
            key
        }
    }

prop_compose! {
        fn arbitrary_key_and_range_half()
            (range in arbitrary_range_half())
            (key in arbitrary_key_in_range_half(&range), range in Just(range))
            -> (usize, RangeHalf) {
            (key, range)
        }
    }

proptest! {
        #[test]
        fn test_range_open((key, range) in arbitrary_key_and_range_open()) {
            prop_assert!(range.contains(key));
        }

        #[test]
        fn test_range_half((key, range) in arbitrary_key_and_range_half()) {
            prop_assert!(range.contains(key));
        }
    }

