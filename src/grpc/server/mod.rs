use {
    chord_rpc::v1::*,
    chord_rpc::v1::server::Chord,
    chord_rpc::v1::server::ChordServer,
    log::{error, info},
    std::net::SocketAddr,
    std::sync::Arc,
    std::sync::RwLock,
    tokio::{
        executor::DefaultExecutor, net::TcpListener, prelude::*, prelude::future::FutureResult,
    },
    tower_grpc::{Code, Request, Response, Status},
    tower_h2::Server,
};

use crate::keys;
use crate::state::State;

#[derive(Debug, Clone)]
pub struct ChordService {
    inner: Arc<RwLock<State>>,
}

impl ChordService {
    pub fn new() -> Self {
        let inner = Arc::new(RwLock::new(State::new()));
        ChordService { inner }
    }

    pub fn serve(self, addr: &SocketAddr) {
        let service = ChordServer::new(self);
        let mut http2 = Server::new(service, Default::default(), DefaultExecutor::current());

        let serve = TcpListener::bind(addr)
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

impl Chord for ChordService {
    type GetNodeFuture = FutureResult<Response<Node>, Status>;
    type UpdateNodeFuture = FutureResult<Response<Node>, Status>;
    type ListKeysFuture = FutureResult<Response<ListKeysResponse>, Status>;
    type GetKeyFuture = FutureResult<Response<KeyMeta>, Status>;
    type CreateKeyFuture = FutureResult<Response<KeyMeta>, Status>;
    type UpdateKeyFuture = FutureResult<Response<KeyMeta>, Status>;
    type DeleteKeyFuture = FutureResult<Response<EmptyResponse>, Status>;

    // nodes

    fn get_node(&mut self, request: Request<EmptyRequest>) -> Self::GetNodeFuture {
        let response = Response::new(self.inner.read().unwrap().node.clone());
        future::ok(response)
    }

    fn update_node(&mut self, request: Request<UpdateNodeRequest>) -> Self::UpdateNodeFuture {
        if request.get_ref().node.is_none() {
            return future::err(Status::with_code(Code::InvalidArgument));
        }

        let node = request.get_ref().node.as_ref().unwrap();
        let mask = request.get_ref().update_mask.as_ref().unwrap();

        {
            let mut n = self.inner.write().unwrap();
            for field in mask.paths.iter() {
                if field == "state" {
                    let rs = RunState::from_i32(node.state).unwrap();
                    n.node.set_state(rs);
                }
            }
        }

        let response = Response::new(self.inner.read().unwrap().node.clone());
        future::ok(response)
    }

    // keys

    fn list_keys(&mut self, request: Request<ListKeysRequest>) -> Self::ListKeysFuture {
        let s = self.inner.read().unwrap();
        let keymetas = s.keys.values().map(|km| km.clone()).collect();
        let size = s.keys.len();

        let resp = ListKeysResponse {
            keys: keymetas,
            next_page_token: String::from("token"),
            total_size: size as i32,
        };
        let response = Response::new(resp);
        future::ok(response)
    }

    fn get_key(&mut self, request: Request<GetKeyRequest>) -> Self::GetKeyFuture {
        let name = &request.get_ref().name;

        let s = self.inner.read().unwrap();
        if let Some(keymeta) = s.keys.get(name) {
            let response = Response::new(keymeta.clone());
            future::ok(response)
        } else {
            future::err(Status::with_code(Code::NotFound))
        }
    }

    /// fixme: this also updates if the key already existed; should it?
    fn create_key(&mut self, request: Request<CreateKeyRequest>) -> Self::CreateKeyFuture {
        if request.get_ref().key.is_none() {
            return future::err(Status::with_code(Code::InvalidArgument));
        }

        let key = request.into_inner().key.unwrap();
        let keymeta = keys::keymeta(key);

        {
            let mut s = self.inner.write().unwrap();
            s.keys.insert(keymeta.name.clone(), keymeta.clone());
        }

        let response = Response::new(keymeta);
        future::ok(response)
    }

    fn update_key(&mut self, request: Request<UpdateKeyRequest>) -> Self::UpdateKeyFuture {
        //        let response = Response::new(KeyMeta::default());
        future::err(Status::with_code(Code::Unimplemented))
    }

    fn delete_key(&mut self, request: Request<DeleteKeyRequest>) -> Self::DeleteKeyFuture {
        let name = &request.get_ref().name;

        let mut s = self.inner.write().unwrap();
        if let Some(_) = s.keys.remove(name) {
            let response = Response::new(EmptyResponse {});
            future::ok(response)
        } else {
            future::err(Status::with_code(Code::NotFound))
        }
    }
}
