use {
    quicli::prelude::*,
    structopt::StructOpt,
};

use chord::grpc::server::ChordService;

#[derive(StructOpt, Debug)]
#[structopt(name = "chord")]
struct ChordCli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    node(NodeCmd),
    keys(KeysCmd),
}

#[derive(StructOpt, Debug)]
enum NodeCmd {
    start,
    info,
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

    std::env::set_var("RUST_LOG", "chord=debug");
    pretty_env_logger::init();

    let srv = ChordService::new();
    srv.serve(&"[::1]:32031".parse().unwrap())
}
