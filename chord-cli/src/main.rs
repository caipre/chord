use quicli::prelude::*;
use structopt::StructOpt;

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
}
