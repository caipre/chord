use std::collections::HashMap;

use proptest::prelude::*;
use proptest::*;

use super::*;

fn fixture_figure_3b() -> HashMap<usize, Node> {
    let mut m = HashMap::new();

    let node = Node::new(
        0,
        vec![Entry::new(1, 1), Entry::new(2, 3), Entry::new(4, 0)],
    );
    m.insert(node.id, node);

    let node = Node::new(
        1,
        vec![Entry::new(2, 3), Entry::new(3, 3), Entry::new(5, 0)],
    );
    m.insert(node.id, node);

    let node = Node::new(
        3,
        vec![Entry::new(4, 0), Entry::new(5, 0), Entry::new(7, 0)],
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
