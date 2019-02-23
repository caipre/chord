use {
    chord_rpc::v1::*,
    std::collections::HashMap,
};

#[derive(Debug)]
pub struct State {
    pub node: Node,
    pub keys: HashMap<String, Vec<u8>>,
}

impl State {
    pub fn new() -> Self {
        State {
            node: Node::default(),
            keys: HashMap::new(),
        }
    }
}
