use {
    http::Uri,
    log::error,
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
use chord_rpc::v1::client::Chord;

pub struct Client {
    client: Chord<AddOrigin<Connection<TcpStream, DefaultExecutor, BoxBody>>>,
}

impl Client {
    pub fn new(addr: &SocketAddr, origin: Uri) -> Self {
        let client = TcpStream::connect(addr)
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

                Chord::new(conn)
            })
            .wait()
            .unwrap();

        Client { client }
    }
}
