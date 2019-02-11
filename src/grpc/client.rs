use {
    tokio::{
        executor::DefaultExecutor,
        net::TcpStream,
        prelude::*,
    },
    tower_grpc::BoxBody,
    tower_h2::client::Connection,
    tower_http::AddOrigin,
};

use chord_rpc::v1::client::Chord;

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
