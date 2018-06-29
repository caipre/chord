#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "chord")]
struct Chord {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Control status for a local or remote node.
    #[structopt(name = "node")]
    Node(NodeCmd),
    /// Fetch or update key/value data in the ring.
    #[structopt(name = "keys")]
    Keys(KeysCmd),
}

#[derive(StructOpt, Debug)]
enum NodeCmd {
    #[structopt(name = "start")]
    Start,
    #[structopt(name = "enable")]
    Enable,
    #[structopt(name = "disable")]
    Disable,
    #[structopt(name = "stop")]
    Stop,
}

#[derive(StructOpt, Debug)]
enum KeysCmd {
    #[structopt(name = "list")]
    List,
    #[structopt(name = "get")]
    Get,
    #[structopt(name = "create")]
    Create,
    #[structopt(name = "update")]
    Update,
    #[structopt(name = "delete")]
    Delete,
}

fn main() {
    let chord = Chord::from_args();
    println!("{:?}", chord);
}
