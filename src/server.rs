use std::collections::btree_map::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::RwLock;

use log::{error, info};
use tokio::{executor::DefaultExecutor, net::TcpListener};
use tokio::prelude::*;
use tower_grpc::{Code, Request, Response, Status};
use tower_h2::Server;

use chord_rpc::v1;
use chord_rpc::v1::server::ChordServer;
use future::FutureResult;

#[derive(Debug)]
#[derive(Clone)]
pub struct ChordService {
    pub node: Node,
    state: RunState,
    pub ftab: Vec<Entry>,
    keys: BTreeMap<u64, Vec<u8>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Entry {
    start: u64,
    end: u64,
    pub node: Node,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Node {
    pub id: u64,
    pub addr: SocketAddr,
//    client: Option<ChordClient>
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

impl ChordService {
    pub fn new(addr: SocketAddr) -> Self {
        let id = (addr.port() % 64) as u64;
        let node = Node { id, addr };
        let state = RunState::Starting;
        let ftab = Vec::with_capacity(64);
        let keys = BTreeMap::new();
        ChordService { node, state, ftab, keys }
    }

    pub fn with_entry(mut self, id: u64, addr: SocketAddr, start: u64, end: u64) -> Self {
        let node = Node { id, addr };
        let entry = Entry { start, end, node };
        self.ftab.insert(id as usize, entry);
        self
    }

    pub fn serve(self) {
        let addr = self.node.addr.clone();
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

impl v1::server::Chord for ChordService {
    type GetNodeFuture = FutureResult<Response<v1::Node>, Status>;
    type GetClosestNodeFuture = FutureResult<Response<v1::Node>, Status>;
    type ListKeysFuture = FutureResult<Response<v1::ListKeysResponse>, Status>;
    type GetKeyFuture = FutureResult<Response<v1::Key>, Status>;
    type CreateKeyFuture = FutureResult<Response<v1::Key>, Status>;
    type DeleteKeyFuture = FutureResult<Response<v1::EmptyResponse>, Status>;

    fn get_node(&mut self, request: Request<v1::EmptyRequest>) -> Self::GetNodeFuture {
        let resp = Response::new(self.node.clone().into());
        future::ok(resp)
    }

    fn get_closest_node(&mut self, request: Request<v1::GetClosestNodeRequest>) -> Self::GetClosestNodeFuture {
        let id = request.get_ref().id;
        let resp = Response::new(self.preceding(id).into());
        future::ok(resp)
    }

    fn list_keys(&mut self, request: Request<v1::ListKeysRequest>) -> Self::ListKeysFuture {
        future::err(Status::with_code(Code::Unimplemented))
//        let cn = self.inner.read().unwrap();
//        let keys = cn.keys.values().map(|km| km.clone()).collect();
//        let size = cn.keys.len();
//
//        let resp = v1::ListKeysResponse {
//            keys: keys,
//            next_page_token: String::from("token"),
//            total_size: size as i32,
//        };
//        let response = Response::new(resp);
//        future::ok(response)
    }

    fn get_key(&mut self, request: Request<v1::GetKeyRequest>) -> Self::GetKeyFuture {
        future::err(Status::with_code(Code::Unimplemented))
//        let name = &request.get_ref().name;
//
//        let cn = self.inner.read().unwrap();
//        if let Some(keymeta) = cn.keys.get(name) {
//            let response = Response::new(keymeta.clone());
//            future::ok(response)
//        } else {
//            future::err(Status::with_code(Code::NotFound))
//        }
    }

    /// fixme: this also updates if the key already existed; should it?
    fn create_key(&mut self, request: Request<v1::CreateKeyRequest>) -> Self::CreateKeyFuture {
        future::err(Status::with_code(Code::Unimplemented))
//        if request.get_ref().key.is_none() {
//            return future::err(Status::with_code(Code::InvalidArgument));
//        }
//
//        let key = request.into_inner().key.unwrap();
//        let keymeta = keys::keymeta(key);
//
//        {
//            let mut cn = self.inner.write().unwrap();
//            cn.keys.insert(keymeta.name.clone(), keymeta.clone());
//        }
//
//        let response = Response::new(keymeta);
//        future::ok(response)
    }


    fn delete_key(&mut self, request: Request<v1::DeleteKeyRequest>) -> Self::DeleteKeyFuture {
        future::err(Status::with_code(Code::Unimplemented))
//        let name = &request.get_ref().name;
//
//        let mut cn = self.inner.write().unwrap();
//        if let Some(_) = cn.keys.remove(name) {
//            let response = Response::new(v1::EmptyResponse {});
//            future::ok(response)
//        } else {
//            future::err(Status::with_code(Code::NotFound))
//        }
    }
}


