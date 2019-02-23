use {
    chord_rpc::v1::*,
    chord_rpc::v1::server::Chord,
    chord_rpc::v1::server::ChordServer,
    log::{error, info},
    std::net::SocketAddr,
    std::sync::Arc,
    tokio::{
        executor::DefaultExecutor,
        net::TcpListener,
        prelude::*,
        prelude::future::FutureResult,
    },
    tower_grpc::{Code, Error, Request, Response, Status},
    tower_h2::Server,
};

use crate::state::State;

#[derive(Debug, Clone)]
pub struct ChordService {
    inner: Arc<State>,
}

impl ChordService {
    pub fn new() -> Self {
        let inner = Arc::new(State);
        ChordService { inner }
    }

    pub fn serve(self, addr: &SocketAddr) {
        let service = ChordServer::new(self);
        let mut http2 = Server::new(service, Default::default(), DefaultExecutor::current());

        let serve = TcpListener::bind(addr).unwrap()
            .incoming()
            .map_err(|err| error!("tcp accept failed; err={}", err))
            .for_each(move |sock| {
                sock.set_nodelay(true).unwrap();
                tokio::spawn(
                    http2.serve(sock)
                        .map_err(|err| error!("http/2 failed; err={:?}", err)))
            });

        tokio::run(serve);
    }

    
}


impl Chord for ChordService {
    type GetNodeFuture = FutureResult<Response<Node>, Error>;
    type UpdateNodeFuture = FutureResult<Response<Node>, Error>;
    type ListKeysFuture = FutureResult<Response<ListKeysResponse>, Error>;
    type GetKeyFuture = FutureResult<Response<KeyMeta>, Error>;
    type CreateKeyFuture = FutureResult<Response<KeyMeta>, Error>;
    type UpdateKeyFuture = FutureResult<Response<KeyMeta>, Error>;
    type DeleteKeyFuture = FutureResult<Response<EmptyResponse>, Error>;

    fn get_node(&mut self, request: Request<EmptyRequest>) -> Self::GetNodeFuture {
        let response = Response::new(Node::default());
        future::ok(response)
    }

    fn update_node(&mut self, request: Request<UpdateNodeRequest>) -> Self::UpdateNodeFuture {
        if request.get_ref().node.is_none() {
            return future::err(Error::Grpc(Status::with_code(Code::InvalidArgument)));
        }

        info!("patch node to {:?}", request.get_ref().update_mask);

        let response = Response::new(Node::default());
        future::ok(response)
    }

    fn list_keys(&mut self, request: Request<ListKeysRequest>) -> Self::ListKeysFuture {
        let response = Response::new(ListKeysResponse::default());
        future::ok(response)
    }

    fn get_key(&mut self, request: Request<GetKeyRequest>) -> Self::GetKeyFuture {
        let response = Response::new(KeyMeta::default());
        future::ok(response)
    }

    fn create_key(&mut self, request: Request<CreateKeyRequest>) -> Self::CreateKeyFuture {
        let response = Response::new(KeyMeta::default());
        future::ok(response)
    }

    fn update_key(&mut self, request: Request<UpdateKeyRequest>) -> Self::UpdateKeyFuture {
        let response = Response::new(KeyMeta::default());
        future::ok(response)
    }

    fn delete_key(&mut self, request: Request<DeleteKeyRequest>) -> Self::DeleteKeyFuture {
        let response = Response::new(EmptyResponse {});
        future::ok(response)
    }
}
