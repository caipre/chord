// Chord key resolution logic
//

impl ChordNode {
    /// Return id of node responsible for key.
    fn find_n(&self, key: usize, ring: &HashMap<usize, ChordNode>) -> usize {
        let id = self.find_p(key, ring);
        ring.get(&id).unwrap().next // fixme: this should make an rpc
    }

    /// Return id of node immediately preceding key.
    fn find_p(&self, key: usize, ring: &HashMap<usize, ChordNode>) -> usize {
        let mut n = self;
        while !n.is_p(key) {
            let id = n.next_p(key);
            n = ring.get(&id).unwrap(); // fixme: this should make an rpc
        }
        n.id
    }

    /// Return whether node immediately precedes key.
    fn is_p(&self, key: usize) -> bool {
        Range::half(self.id, self.next).contains(key)
    }

    /// Return id of node near to and preceding key.
    fn next_p(&self, key: usize) -> usize {
        let rg = Range::open(self.id, key);
        for entry in self.tabl.iter().rev() {
            if rg.contains(entry.node) {
                return entry.node;
            }
        }
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeOpen {
    range: Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeHalf {
    range: Range,
}

impl Range {
    fn open(start: usize, end: usize) -> RangeOpen {
        RangeOpen {
            range: Range { start, end },
        }
    }

    fn half(start: usize, end: usize) -> RangeHalf {
        RangeHalf {
            range: Range { start, end },
        }
    }
}

impl RangeOpen {
    /// Return whether key in full-open range (start, end)
    pub fn contains(&self, key: usize) -> bool {
        if self.range.start == self.range.end {
            self.range.start != key
        } else if self.range.start < self.range.end {
            self.range.start < key && key < self.range.end
        } else {
            self.range.start < key || key < self.range.end
        }
    }
}

impl RangeHalf {
    /// Return whether key in half-open range (start, end]
    pub fn contains(&self, key: usize) -> bool {
        if self.range.start == self.range.end {
            self.range.end == key
        } else if self.range.start < self.range.end {
            self.range.start < key && key <= self.range.end
        } else {
            self.range.start < key || key <= self.range.end
        }
    }
}

///

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_figure_3b() -> HashMap<usize, ChordNode> {
        let mut m = HashMap::new();

        let node = ChordNode::new(
            0,
            vec![Entry::new(1, 1), Entry::new(2, 3), Entry::new(4, 0)],
        );
        m.insert(node.id, node);

        let node = ChordNode::new(
            1,
            vec![Entry::new(2, 3), Entry::new(3, 3), Entry::new(5, 0)],
        );
        m.insert(node.id, node);

        let node = ChordNode::new(
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
}

