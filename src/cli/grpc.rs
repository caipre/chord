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

pub fn connect(addr: &'static str) -> impl Future<Item=AddOrigin<Connection<TcpStream, DefaultExecutor, BoxBody>>, Error=()> {
    let dst = Dst::from_str(addr).unwrap();
    let mut h2 = Connect::new(dst, Default::default(), DefaultExecutor::current());
    h2.make_service(())
        .map(move |conn| {
            let uri = http::Uri::builder()
                .scheme(http::uri::Scheme::HTTP)
                .authority(http::uri::Authority::from_str(addr).unwrap())
                .build()
                .unwrap();
            add_origin::Builder::new()
                .uri(uri)
                .build(conn)
                .unwrap()
        })
        .map_err(|err| eprintln!("failed to connect; err={:?}", err))
}

struct Dst {
    addr: SocketAddr,
}


impl FromStr for Dst {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let addr = SocketAddr::from_str(s)?;
        Ok(Dst { addr })
    }
}

impl tokio_connect::Connect for Dst {
    type Connected = TcpStream;
    type Error = ::std::io::Error;
    type Future = ConnectFuture;

    fn connect(&self) -> Self::Future {
        TcpStream::connect(&self.addr)
    }
}
