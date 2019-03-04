use std::net::SocketAddr;

use http::Uri;
use log::{error, info};
use tokio::{executor::DefaultExecutor, net::TcpStream};
use tokio::prelude::*;
use tower_grpc::{BoxBody, Request};
use tower_h2::client::Connection;
use tower_http::AddOrigin;

use chord_rpc::v1;
use chord_rpc::v1::client::Chord;

use super::errors::ClientError;

pub struct ChordClient {
    client: Chord<AddOrigin<Connection<TcpStream, DefaultExecutor, BoxBody>>>,
}

pub fn connect(addr: &SocketAddr, origin: Uri) -> impl Future<Item=ChordClient, Error=()> {
    TcpStream::connect(addr)
        .map_err(|err| error!("tcp connect failed; err={:?}", err))
        .and_then(move |sock| {
            Connection::handshake(sock, DefaultExecutor::current())
                .map_err(|err| error!("http/2 handshake failed; err={:?}", err))
        })
        .map(move |conn| {
            use tower_http::add_origin::Builder;
            let conn = Builder::new().uri(origin).build(conn).unwrap();

            ChordClient {
                client: Chord::new(conn),
            }
        })
}

impl ChordClient {
    pub fn get_node(&mut self) -> impl Future<Item=v1::Node, Error=ClientError> {
        self.client
            .get_node(Request::new(v1::EmptyRequest {}))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn get_closest_peer(&mut self, id: u64) -> impl Future<Item=v1::Peer, Error=ClientError> {
        let req = v1::GetClosestPeerRequest { id };
        self.client
            .get_closest_peer(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    /// fixme: return Stream<KeyMeta> and automatically make next request
    pub fn list_keys(&mut self) -> impl Future<Item=v1::ListKeysResponse, Error=ClientError> {
        let req = v1::ListKeysRequest {
            page_size: 100,
            page_token: String::from(""),
        };
        self.client
            .list_keys(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn get_key(&mut self, name: &str) -> impl Future<Item=v1::Key, Error=ClientError> {
        let req = v1::GetKeyRequest {
            name: String::from(name),
        };
        self.client
            .get_key(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn create_key(&mut self, key: v1::Key) -> impl Future<Item=v1::Key, Error=ClientError> {
        let req = v1::CreateKeyRequest { key: Some(key) };
        self.client
            .create_key(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| resp.into_inner())
    }

    pub fn delete_key(&mut self, name: &str) -> impl Future<Item=(), Error=ClientError> {
        let req = v1::DeleteKeyRequest {
            name: String::from(name),
        };
        self.client
            .delete_key(Request::new(req))
            .map_err(ClientError::from)
            .map(|resp| ())
    }
}
