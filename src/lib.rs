use std::net::SocketAddr;

use log::{error, info};
use tokio::{executor::DefaultExecutor, net::TcpListener};
use tokio::prelude::*;
use tower_h2::Server;

use chord_rpc::v1::server::ChordServer;

mod client;
mod server;

mod convert;
mod errors;
mod resolve;
mod storage;

type KeyType = u64;

#[derive(Debug)]
#[derive(Clone)]
pub struct ChordService {
    id: KeyType,
    addr: SocketAddr,
    state: RunState,
    entries: Vec<Entry>,
}

impl ChordService {
    pub fn new(id: KeyType, addr: SocketAddr) -> Self {
        let state = RunState::Starting;
        let entries = Vec::with_capacity(std::mem::size_of::<KeyType>());
        ChordService { id, addr, state, entries }
    }

    pub fn with_entry(mut self, id: KeyType, addr: SocketAddr, start: KeyType, end: KeyType) -> Self {
        let peer = Node { id, addr };
        let entry = Entry { start, end, node: peer };
        self.entries.insert(id as usize, entry);
        self
    }

    pub fn serve(self) {
        let addr = self.addr.clone();
        let service = ChordServer::new(self);
        let mut http2 = Server::new(service, Default::default(), DefaultExecutor::current());

        let serve = TcpListener::bind(&addr)
            .unwrap()
            .incoming()
            .map_err(|err| error!("tcp accept failed; err={}", err))
            .for_each(move |sock| {
                sock.set_nodelay(true).unwrap();
                tokio::spawn(
                    http2
                        .serve(sock)
                        .map_err(|err| error!("http/2 failed; err={:?}", err)),
                )
            });

        tokio::run(serve);
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Node {
    id: u64,
    addr: SocketAddr,
//    client: Option<ChordClient>
}

#[derive(Debug)]
#[derive(Clone)]
struct Entry {
    start: u64,
    end: u64,
    node: Node,
}

#[derive(Debug)]
#[derive(Clone)]
enum RunState {
    Starting,
    Waiting(String),
    Ready,
    Running,
    Stopping,
}
