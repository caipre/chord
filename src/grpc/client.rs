use {
    chord_rpc::v1::*,
    chord_rpc::v1::client::Chord,
    http::Uri,
    log::{error, info},
    std::fmt,
    std::net::SocketAddr,
    tokio::{
        executor::DefaultExecutor,
        net::TcpStream,
        prelude::*,
    },
    tokio::sync::oneshot,
    tower_grpc::{BoxBody, Request, Response},
    tower_h2::client::Connection,
    tower_http::AddOrigin,
};

type ChordClient = Chord<AddOrigin<Connection<TcpStream, DefaultExecutor, BoxBody>>>;

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

            Chord::new(conn)
        })
}

//

#[derive(Debug)]
pub enum ClientError {
    GrpcError(tower_grpc::Error),
    HttpError(tower_grpc::Error<tower_h2::client::Error>),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ClientError {}

impl From<tower_grpc::Error> for ClientError {
    fn from(err: tower_grpc::Error) -> Self {
        ClientError::GrpcError(err)
    }
}

impl From<tower_grpc::Error<tower_h2::client::Error>> for ClientError {
    fn from(err: tower_grpc::Error<tower_h2::client::Error>) -> Self {
        ClientError::HttpError(err)
    }
}

// node

pub fn get_node(mut client: ChordClient) -> impl Future<Item=Node, Error=ClientError> {
    client.get_node(Request::new(EmptyRequest {}))
        .map_err(ClientError::from)
        .map(|resp| resp.into_inner())
}

// keys
