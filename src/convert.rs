use chord_rpc::v1;

impl From<crate::Node> for v1::Peer {
    fn from(from: crate::Node) -> Self {
        v1::Peer {
            id: from.id,
            addr: from.addr.to_string(),
        }
    }
}

impl From<&crate::ChordService> for crate::Node {
    fn from(from: &crate::ChordService) -> Self {
        crate::Node { id: from.id, addr: from.addr }
    }
}
