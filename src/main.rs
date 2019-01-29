use futures::future::Future;
use futures::stream::Stream;
use quicli::prelude::*;
use structopt::StructOpt;
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_h2::Server;

use chord::rpc::v1::server::ChordServer;
use chord::srv::grpc::ChordService;

#[derive(StructOpt, Debug)]
#[structopt(name = "chord")]
struct ChordCli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Control status for a local or remote node.
    node(NodeCmd),
    /// Fetch or update key/value data in the ring.
    keys(KeysCmd),
}

#[derive(StructOpt, Debug)]
enum NodeCmd {
    start,
    enable,
    disable,
    stop,
}

#[derive(StructOpt, Debug)]
enum KeysCmd {
    list,
    get,
    create,
    update,
    delete,
}

fn main() {
    let chord = ChordCli::from_args();

    let service = ChordServer::new(ChordService);
    let h2 = Server::new(service, Default::default(), DefaultExecutor::current());

    let addr = "[::1]:32031".parse().unwrap();
    let bind = TcpListener::bind(&addr).unwrap();

    let serve = bind.incoming()
        .fold(h2, |mut h2, sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            tokio::spawn({
                h2.serve(sock)
                    .map_err(|e| error!("h2 error: {:?}", e))
            });

            Ok(h2)
        })
        .map_err(|e| eprintln!("accept error: {}", e))
        .map(|_| {});
    tokio::run(serve);
}
