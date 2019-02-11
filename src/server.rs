use std::net::ToSocketAddrs;

use futures::future;
use futures::future::Future;
use futures::future::FutureResult;
use futures::stream::Stream;
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_grpc::{Code, Error, Status};
use tower_grpc::{Request, Response};
use tower_h2::Server;

use crate::rpc::v1::server::Chord;
use crate::rpc::v1::server::ChordServer;
use crate::rpc::v1::CreateKeyRequest;
use crate::rpc::v1::DeleteKeyRequest;
use crate::rpc::v1::GetKeyRequest;
use crate::rpc::v1::UpdateKeyRequest;
use crate::rpc::v1::UpdateNodeRequest;
use crate::rpc::v1::{EmptyRequest, EmptyResponse};
use crate::rpc::v1::{Key, KeyMeta, Node};
use crate::rpc::v1::{ListKeysRequest, ListKeysResponse};

#[derive(Clone)]
pub struct ChordService;

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

pub fn start<A: ToSocketAddrs>(_addrs: A) {
    let service = ChordServer::new(ChordService);
    let h2 = Server::new(service, Default::default(), DefaultExecutor::current());

    let addr = "[::1]:32031".parse().unwrap();
    let bind = TcpListener::bind(&addr).unwrap();

    let serve = bind
        .incoming()
        .fold(h2, |mut h2, sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            println!("incoming connection: {:?}", sock);

            tokio::spawn({ h2.serve(sock).map_err(|e| eprintln!("h2 error: {:?}", e)) });

            Ok(h2)
        })
        .map_err(|e| eprintln!("accept error: {}", e))
        .map(|_| {});
    tokio::run(serve);
}
