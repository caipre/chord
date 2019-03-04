use future::FutureResult;
use tokio::prelude::*;
use tower_grpc::{Code, Request, Response, Status};

use chord_rpc::v1;

use super::ChordService;

impl v1::server::Chord for ChordService {
    type GetNodeFuture = FutureResult<Response<v1::Node>, Status>;
    type GetClosestPeerFuture = FutureResult<Response<v1::Peer>, Status>;
    type ListKeysFuture = FutureResult<Response<v1::ListKeysResponse>, Status>;
    type GetKeyFuture = FutureResult<Response<v1::Key>, Status>;
    type CreateKeyFuture = FutureResult<Response<v1::Key>, Status>;
    type DeleteKeyFuture = FutureResult<Response<v1::EmptyResponse>, Status>;

    fn get_node(&mut self, request: Request<v1::EmptyRequest>) -> Self::GetNodeFuture {
        future::err(Status::with_code(Code::Unimplemented))
//        let response = Response::new(self.inner.read().unwrap().clone());
//        future::ok(response)
    }

    fn get_closest_peer(&mut self, request: Request<v1::GetClosestPeerRequest>) -> Self::GetClosestPeerFuture {
        future::err(Status::with_code(Code::Unimplemented))
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


