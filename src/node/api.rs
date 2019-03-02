use std::net::SocketAddr;

use chord_rpc::v1::KeyMeta;

pub struct Error;

#[derive(Debug)]
pub struct ChordNode {
    id: u64,
    state: RunState,
    next: SocketAddr,
    entries: Vec<Entry>,
}

impl ChordNode {
    pub fn new(id: u64, entries: Vec<Entry>) -> Self {
        let next = entries.first().map(|e| e.addr).unwrap();
        let state = RunState::Starting;
        ChordNode { id, state, next, entries }
    }

    // state mgmt
    pub fn update(&mut self) {
        unimplemented!()
    }

    // key fns

    pub fn create(&mut self, key &str) -> Result<KeyMeta, Error> {
        unimplemented!()
    }

    pub fn list(&mut self, key: &str) -> Result<KeyMeta, Error> {
        unimplemented!()
    }

    pub fn find(&mut self, key: &str) -> Result<KeyMeta, Error> {
        unimplemented!()
    }

    pub fn replace(&mut self, key &str) -> Result<KeyMeta, Error> {
        unimplemented!()
    }

    pub fn delete(&mut self, key &str) -> Result<KeyMeta, Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum RunState {
    Starting,
    Waiting(String),
    Ready,
    Running,
    Stopping,
}

#[derive(Debug)]
pub struct Entry {
    addr: SocketAddr,
    start: u64,
    node: u64,
}

impl Entry {
    pub fn new(addr: SocketAddr, start: u64, node: u64) -> Self {
        Entry { addr, start, node }
    }
}

