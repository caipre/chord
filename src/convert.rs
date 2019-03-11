use chord_rpc::v1;

impl From<crate::server::Node> for v1::Node {
    fn from(from: crate::server::Node) -> Self {
        v1::Node {
            id: from.id,
            addr: from.addr.to_string(),
        }
    }
}

impl From<&crate::ChordService> for crate::server::Node {
    fn from(from: &crate::ChordService) -> Self {
        crate::server::Node { id: from.node.id, addr: from.node.addr }
    }
}
