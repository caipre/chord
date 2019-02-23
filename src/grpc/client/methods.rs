use {
    chord_rpc::v1::*,
    log::{error, info},
    prost_types::FieldMask,
    tokio::prelude::Future,
    tower_grpc::Request,
};

use crate::grpc::client::errors::ClientError;

use super::ChordClient;

// node
impl ChordClient {
    pub fn get_node(&mut self) -> impl Future<Item = Node, Error = ClientError> {
        self.client
            .get_node(Request::new(EmptyRequest {}))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn update_node(
        &mut self,
        node: Node,
        mask: FieldMask,
    ) -> impl Future<Item = Node, Error = ClientError> {
        let req = UpdateNodeRequest {
            node: Some(node),
            update_mask: Some(mask),
        };
        self.client
            .update_node(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }
}

// keys
impl ChordClient {
    /// fixme: return Stream<KeyMeta> and automatically make next request
    pub fn list_keys(&mut self) -> impl Future<Item = ListKeysResponse, Error = ClientError> {
        let req = ListKeysRequest {
            page_size: 100,
            page_token: String::from(""),
        };
        self.client
            .list_keys(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn get_key(&mut self, name: &str) -> impl Future<Item = KeyMeta, Error = ClientError> {
        let req = GetKeyRequest {
            name: String::from(name),
        };
        self.client
            .get_key(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn create_key(&mut self, key: Key) -> impl Future<Item = KeyMeta, Error = ClientError> {
        let req = CreateKeyRequest { key: Some(key) };
        self.client
            .create_key(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn update_key(
        &mut self,
        key: Key,
        mask: FieldMask,
    ) -> impl Future<Item = KeyMeta, Error = ClientError> {
        let req = UpdateKeyRequest {
            key: Some(key),
            update_mask: Some(mask),
        };
        self.client
            .update_key(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn delete_key(&mut self, name: &str) -> impl Future<Item = (), Error = ClientError> {
        let req = DeleteKeyRequest {
            name: String::from(name),
        };
        self.client
            .delete_key(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| ())
    }
}
