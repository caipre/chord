use {
    chord_rpc::v1::client::Chord,
    http::Uri,
    log::{error, info},
    std::net::SocketAddr,
    tokio::{
        executor::DefaultExecutor,
        net::TcpStream,
        prelude::*,
    },
    tower_grpc::BoxBody,
    tower_h2::client::Connection,
    tower_http::AddOrigin,
};

mod errors;
mod methods;

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
            let conn = Builder::new()
                .uri(origin)
                .build(conn)
                .unwrap();

            ChordClient { client: Chord::new(conn) }
        })
}
