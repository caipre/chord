use futures::future::FutureResult;
use tower_grpc::{Error, Request, Response};

use crate::rpc::v1::{Node, Key, KeyMeta};
use crate::rpc::v1::{EmptyRequest, EmptyResponse};
use crate::rpc::v1::{ListKeysRequest, ListKeysResponse};
use crate::rpc::v1::CreateKeyRequest;
use crate::rpc::v1::DeleteKeyRequest;
use crate::rpc::v1::GetKeyRequest;
use crate::rpc::v1::server::Chord;
use crate::rpc::v1::UpdateKeyRequest;
use crate::rpc::v1::UpdateNodeRequest;

#[derive(Clone)]
struct ChordService;

impl Chord for ChordService {
    type GetNodeFuture = FutureResult<Response<Node>, Error>;
    type UpdateNodeFuture = FutureResult<Response<Node>, Error>;
    type ListKeysFuture = FutureResult<Response<ListKeysResponse>, Error>;
    type GetKeyFuture = FutureResult<Response<KeyMeta>, Error>;
    type CreateKeyFuture = FutureResult<Response<KeyMeta>, Error>;
    type UpdateKeyFuture = FutureResult<Response<KeyMeta>, Error>;
    type DeleteKeyFuture = FutureResult<Response<EmptyResponse>, Error>;

    fn get_node(&mut self, request: Request<EmptyRequest>) -> Self::GetNodeFuture {
        unimplemented!()
    }

    fn update_node(&mut self, request: Request<UpdateNodeRequest>) -> Self::UpdateNodeFuture {
        unimplemented!()
    }

    fn list_keys(&mut self, request: Request<ListKeysRequest>) -> Self::ListKeysFuture {
        unimplemented!()
    }

    fn get_key(&mut self, request: Request<GetKeyRequest>) -> Self::GetKeyFuture {
        unimplemented!()
    }

    fn create_key(&mut self, request: Request<CreateKeyRequest>) -> Self::CreateKeyFuture {
        unimplemented!()
    }

    fn update_key(&mut self, request: Request<UpdateKeyRequest>) -> Self::UpdateKeyFuture {
        unimplemented!()
    }

    fn delete_key(&mut self, request: Request<DeleteKeyRequest>) -> Self::DeleteKeyFuture {
        unimplemented!()
    }
}
