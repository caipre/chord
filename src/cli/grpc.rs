use std::io;
use std::net::SocketAddr;
use std::net::SocketAddrV6;
use std::net::ToSocketAddrs;
use std::str::FromStr;

use futures::Future;
use tokio::executor::DefaultExecutor;
use tokio::net::tcp::ConnectFuture;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tower_grpc::BoxBody;
use tower_grpc::Request;
use tower_h2::client::Connect;
use tower_h2::client::Connection;
use tower_http::add_origin;
use tower_http::add_origin::AddOrigin;
use tower_util::MakeService;

use crate::rpc::v1::client::Chord;

pub struct Client {
    client: Chord<AddOrigin<Connection<TcpStream, DefaultExecutor, BoxBody>>>,
}

impl Client {
    pub fn new() -> Self {
        let addr = "[::1]:32031".parse().unwrap();
        let uri: http::Uri = "http://localhost:32031".parse().unwrap();

        let client = TcpStream::connect(&addr)
            .and_then(move |sock| {
                Connection::handshake(sock, DefaultExecutor::current())
                    .map_err(|err| panic!("http/2 handshake failed; err={:?}", err))
            })
            .map(move |conn| {
                use tower_http::add_origin::Builder;
                let conn = Builder::new().uri(uri).build(conn).unwrap();

                Chord::new(conn)
            })
            .map_err(|err| eprintln!("connect failed; err={:?}", err))
            .wait()
            .unwrap();

        Client { client }
    }
}
