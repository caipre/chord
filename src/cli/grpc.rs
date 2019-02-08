use std::net::SocketAddrV6;

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

pub fn new() {
    let uri: http::Uri = "http://[::1]:32031".parse().unwrap();

    let mut h2 = Connect::new(Dst, Default::default(), DefaultExecutor::current());
    let request = h2.make_service(())
        .map(move |conn| {
            use crate::rpc::v1::client::Chord;
            use tower_http::add_origin;

            let conn = add_origin::Builder::new()
                .uri(uri)
                .build(conn)
                .unwrap();

            Chord::new(conn)
        })
        .and_then(|mut client| {
            use crate::rpc::v1::EmptyRequest;

            client.get_node(Request::new(EmptyRequest {}))
                .map_err(|err| panic!("grpc request failed; err={:?}", err))
        })
        .and_then(|response| {
            println!("resp={:?}", response);
            Ok(())
        })
        .map_err(|err| println!("resp failed; err={:?}", err));

    tokio::run(request);
}

pub fn client() -> impl Future<Item=Chord<AddOrigin<Connection<TcpStream, DefaultExecutor, BoxBody>>>, Error=()> {
    let mut h2 = Connect::new(Dst, Default::default(), DefaultExecutor::current());
    h2.make_service(())
        .map(move |conn| {
            let conn = add_origin::Builder::new()
                .uri("http://[::1]:32031".parse::<http::Uri>().unwrap())
                .build(conn)
                .unwrap();

            Chord::new(conn)
        })
        .map_err(|err| eprintln!("failed to connect; err={:?}", err))
}

struct Dst;

impl tokio_connect::Connect for Dst {
    type Connected = TcpStream;
    type Error = ::std::io::Error;
    type Future = ConnectFuture;

    fn connect(&self) -> Self::Future {
        TcpStream::connect(&"[::1]:32031".parse::<SocketAddrV6>().unwrap().into())
    }
}
