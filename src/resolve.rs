// Chord key resolution logic
//

use tokio::prelude::*;
use tower_grpc::Code;
use tower_grpc::Status;

use super::ChordService;
use super::client::ChordClient;
use super::errors::ClientError;
use super::server::Node;

impl ChordService {
    /// Return a client connected to the node responsible for an id.
    fn find(&mut self, id: u64) -> impl Future<Item=ChordClient, Error=ClientError> {
        future::err(ClientError::GrpcError(Status::with_code(Code::Unimplemented)))
//        self.predecessor(id)
//            .and_then(|mut client| client.successor())
//            .flatten()
    }

    /// Return a client connected to the node immediately preceding an id.
    fn predecessor(&mut self, id: u64) -> impl Future<Item=ChordClient, Error=ClientError> {
        future::err(ClientError::GrpcError(Status::with_code(Code::Unimplemented)))
//        self.client_for(self.into())
//            .and_then(move |mut client| {
//                let mut n = client.get_node().wait().unwrap(); //fixme: blocking
//                while !Range::half_open(n.id, n.successor.id).contains(id) {
//                    n = client.get_preceding(id);
//                }
//            })
    }

    /// Return the node close to and preceding an id.
    pub fn preceding(&self, id: u64) -> Node {
        for entry in self.ftab.iter().rev() {
            if Range::full_open(self.node.id, id).contains(entry.node.id) {
                return entry.node;
            }
        }
        self.into()
    }

    /// Return a client connected to the successor of a node.
    fn successor(&mut self, node: Node) -> impl Future<Item=ChordClient, Error=ClientError> {
        future::err(ClientError::GrpcError(Status::with_code(Code::Unimplemented)))
//        self.client_for(node)
//            .and_then(|mut client| client.get_node()).flatten()
//            .and_then(|v1node| self.client_for(v1node.successor.into()))
//            .flatten()
    }

    fn client_for(&mut self, node: Node) -> impl Future<Item=ChordClient, Error=()> {
        future::err(())
//        // fixme: shouldn't need a new client every time
//        let origin = self.addr.into();
//        grpc::client::connect(&node.addr, origin)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}


impl Range {
    fn half_open(start: u64, end: u64) -> RangeHalfOpen {
        RangeHalfOpen(Range { start, end })
    }
    fn full_open(start: u64, end: u64) -> RangeFullOpen {
        RangeFullOpen(Range { start, end })
    }
}

struct RangeFullOpen(Range);

impl RangeFullOpen {
    /// Return whether id in full-open range (start, end)
    fn contains(&self, id: u64) -> bool {
        if self.0.start == self.0.end {
            self.0.start != id
        } else if self.0.start < self.0.end {
            self.0.start < id && id < self.0.end
        } else {
            self.0.start < id || id < self.0.end
        }
    }
}

struct RangeHalfOpen(Range);

impl RangeHalfOpen {
    /// Return whether id in half-open range (start, end]
    fn contains(&self, id: u64) -> bool {
        if self.0.start == self.0.end {
            self.0.end == id
        } else if self.0.start < self.0.end {
            self.0.start < id && id <= self.0.end
        } else {
            self.0.start < id || id <= self.0.end
        }
    }
}

//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    fn fixture_figure_3b() -> HashMap<u64, ChordNode> {
//        let mut m = HashMap::new();
//
//        let node = ChordNode::new(
//            0,
//            vec![Entry::new(1, 1), Entry::new(2, 3), Entry::new(4, 0)],
//        );
//        m.insert(node.id, node);
//
//        let node = ChordNode::new(
//            1,
//            vec![Entry::new(2, 3), Entry::new(3, 3), Entry::new(5, 0)],
//        );
//        m.insert(node.id, node);
//
//        let node = ChordNode::new(
//            3,
//            vec![Entry::new(4, 0), Entry::new(5, 0), Entry::new(7, 0)],
//        );
//        m.insert(node.id, node);
//
//        m
//    }
//
//    #[test]
//    fn test_figure_3b() {
//        let m = fixture_figure_3b();
//        let zer = m.get(&0).unwrap();
//        let one = m.get(&1).unwrap();
//        let thr = m.get(&3).unwrap();
//
//        assert_eq!(zer.next_p(0), 3);
//        assert_eq!(zer.next_p(1), 0);
//        assert_eq!(zer.next_p(2), 1);
//        assert_eq!(zer.next_p(3), 1);
//        assert_eq!(zer.next_p(4), 3);
//        assert_eq!(zer.next_p(5), 3);
//        assert_eq!(zer.next_p(6), 3);
//        assert_eq!(zer.next_p(7), 3);
//
//        assert_eq!(one.next_p(0), 3);
//        assert_eq!(one.next_p(1), 0);
//        assert_eq!(one.next_p(2), 1);
//        assert_eq!(one.next_p(3), 1);
//        assert_eq!(one.next_p(4), 3);
//        assert_eq!(one.next_p(5), 3);
//        assert_eq!(one.next_p(6), 3);
//        assert_eq!(one.next_p(7), 3);
//
//        assert_eq!(thr.next_p(0), 3);
//        assert_eq!(thr.next_p(1), 0);
//        assert_eq!(thr.next_p(2), 0);
//        assert_eq!(thr.next_p(3), 0);
//        assert_eq!(thr.next_p(4), 3);
//        assert_eq!(thr.next_p(5), 3);
//        assert_eq!(thr.next_p(6), 3);
//        assert_eq!(thr.next_p(7), 3);
//
//        assert_eq!(zer.find_p(0, &m), 3);
//        assert_eq!(zer.find_p(1, &m), 0);
//        assert_eq!(zer.find_p(2, &m), 1);
//        assert_eq!(zer.find_p(3, &m), 1);
//        assert_eq!(zer.find_p(4, &m), 3);
//        assert_eq!(zer.find_p(5, &m), 3);
//        assert_eq!(zer.find_p(6, &m), 3);
//        assert_eq!(zer.find_p(7, &m), 3);
//
//        assert_eq!(one.find_p(0, &m), 3);
//        assert_eq!(one.find_p(1, &m), 0);
//        assert_eq!(one.find_p(2, &m), 1);
//        assert_eq!(one.find_p(3, &m), 1);
//        assert_eq!(one.find_p(4, &m), 3);
//        assert_eq!(one.find_p(5, &m), 3);
//        assert_eq!(one.find_p(6, &m), 3);
//        assert_eq!(one.find_p(7, &m), 3);
//
//        assert_eq!(thr.find_p(0, &m), 3);
//        assert_eq!(thr.find_p(1, &m), 0);
//        assert_eq!(thr.find_p(2, &m), 1);
//        assert_eq!(thr.find_p(3, &m), 1);
//        assert_eq!(thr.find_p(4, &m), 3);
//        assert_eq!(thr.find_p(5, &m), 3);
//        assert_eq!(thr.find_p(6, &m), 3);
//        assert_eq!(thr.find_p(7, &m), 3);
//
//        assert_eq!(zer.find_n(0, &m), 0);
//        assert_eq!(zer.find_n(1, &m), 1);
//        assert_eq!(zer.find_n(2, &m), 3);
//        assert_eq!(zer.find_n(3, &m), 3);
//        assert_eq!(zer.find_n(4, &m), 0);
//        assert_eq!(zer.find_n(5, &m), 0);
//        assert_eq!(zer.find_n(6, &m), 0);
//        assert_eq!(zer.find_n(7, &m), 0);
//
//        assert_eq!(one.find_n(0, &m), 0);
//        assert_eq!(one.find_n(1, &m), 1);
//        assert_eq!(one.find_n(2, &m), 3);
//        assert_eq!(one.find_n(3, &m), 3);
//        assert_eq!(one.find_n(4, &m), 0);
//        assert_eq!(one.find_n(5, &m), 0);
//        assert_eq!(one.find_n(6, &m), 0);
//        assert_eq!(one.find_n(7, &m), 0);
//
//        assert_eq!(thr.find_n(0, &m), 0);
//        assert_eq!(thr.find_n(1, &m), 1);
//        assert_eq!(thr.find_n(2, &m), 3);
//        assert_eq!(thr.find_n(3, &m), 3);
//        assert_eq!(thr.find_n(4, &m), 0);
//        assert_eq!(thr.find_n(5, &m), 0);
//        assert_eq!(thr.find_n(6, &m), 0);
//        assert_eq!(thr.find_n(7, &m), 0);
//    }
//
//    #[test]
//    fn test_range_contains() {
//        let modulo = 8;
//        for start in 0..modulo {
//            for end in (start + 1)..(start + modulo) {
//                let endmod = end % modulo;
//                let open = Range::open(start, endmod);
//                let half = Range::half(start, endmod);
//                assert!(!open.contains(start));
//                assert!(!half.contains(start));
//                for i in (start + 1)..end {
//                    let imod = i % modulo;
//                    assert!(open.contains(imod));
//                    assert!(half.contains(imod));
//                }
//                assert!(!open.contains(endmod));
//                assert!(half.contains(endmod));
//                for i in (end + 1)..(start + modulo) {
//                    let imod = i % modulo;
//                    assert!(!open.contains(imod));
//                    assert!(!half.contains(imod));
//                }
//            }
//        }
//    }
//}

