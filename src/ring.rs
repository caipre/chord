use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    uri: String,
    start: usize,
    node: usize,
}

impl Entry {
    fn new(start: usize, node: usize) -> Entry {
        Entry {
            uri: String::from(""),
            start,
            node,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    id: usize,
    state: RunState,
    next: usize,
    tabl: Vec<Entry>,
}

#[derive(Debug)]
enum RunState {
    Starting,
    Waiting(String),
    Ready,
    Running,
    Stopping,
}

impl Node {
    fn new(id: usize, tabl: Vec<Entry>) -> Node {
        let next = tabl.first().map(|e| e.node).unwrap();
        Node {
            id: id,
            state: RunState::Starting,
            next: next,
            tabl: tabl,
        }
    }
}

// Chord key resolution logic
//
impl Node {
    /// Return id of node responsible for key.
    fn find_n(&self, key: usize, ring: &HashMap<usize, Node>) -> usize {
        let id = self.find_p(key, ring);
        ring.get(&id).unwrap().next // fixme: this should make an rpc
    }

    /// Return id of node immediately preceding key.
    fn find_p(&self, key: usize, ring: &HashMap<usize, Node>) -> usize {
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

trait Contains {
    fn contains(&self, key: usize) -> bool;
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

impl Contains for RangeOpen {
    /// Return whether key in full-open range (start, end)
    fn contains(&self, key: usize) -> bool {
        if self.range.start == self.range.end {
            self.range.start != key
        } else if self.range.start < self.range.end {
            self.range.start < key && key < self.range.end
        } else {
            self.range.start < key || key < self.range.end
        }
    }
}

impl Contains for RangeHalf {
    /// Return whether key in half-open range (start, end]
    fn contains(&self, key: usize) -> bool {
        if self.range.start == self.range.end {
            self.range.end == key
        } else if self.range.start < self.range.end {
            self.range.start < key && key <= self.range.end
        } else {
            self.range.start < key || key <= self.range.end
        }
    }
}
