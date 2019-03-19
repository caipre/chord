use std::collections::HashMap;
use std::net::SocketAddr;

use futures::prelude::*;
use quicli::prelude::*;
use structopt::StructOpt;

use chord::ChordClient;
use chord::ChordService;
use chord_rpc::v1;

#[derive(StructOpt, Debug)]
#[structopt(name = "chord")]
struct ChordCli {
    #[structopt(long, parse(try_from_str))]
    addr: Option<SocketAddr>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
#[allow(non_camel_case_types)]
enum Command {
    node(NodeCmd),
    keys(KeysCmd),
}

#[derive(StructOpt, Debug)]
#[allow(non_camel_case_types)]
enum NodeCmd {
    start,
    info,
}

#[derive(StructOpt, Debug)]
#[allow(non_camel_case_types)]
enum KeysCmd {
    list,
    get(GetKeyCmd),
    create(CreateKeyCmd),
    delete(DeleteKeyCmd),
}

#[derive(StructOpt, Debug)]
struct GetKeyCmd {
    name: String,
}

#[derive(StructOpt, Debug)]
struct CreateKeyCmd {
    name: String,
}

#[derive(StructOpt, Debug)]
struct DeleteKeyCmd {
    name: String,
}

fn main() {
    let chord = ChordCli::from_args();

    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let task = ChordClient::connect(
        &"[::1]:32031".parse().unwrap(),
        "http://localhost:32031".parse().unwrap(),
    );

    match chord.cmd {
        Command::node(NodeCmd::start) => {
            let addr = chord.addr.unwrap_or("[::1]:32031".parse().unwrap());
            let srv = ChordService::new(addr);
            srv.serve();
        }

        Command::node(NodeCmd::info) => {
            let t = task
                .and_then(move |mut client| {
                    client
                        .get_node()
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                        .and_then(move |_| {
                            client
                                .get_closest_node(2)
                                .map_err(|err| eprintln!("request failed; err={:?}", err))
                                .map(|resp| println!("{:?}", resp))
                        })
                });
            tokio::run(t);
        }

        _ => unimplemented!()

//        Command::keys(KeysCmd::list) => {
//            let t = task
//                .map(move |mut client| {
//                    client
//                        .list_keys()
//                        .map_err(|err| eprintln!("request failed; err={:?}", err))
//                        .map(|resp| println!("{:?}", resp))
//                })
//                .flatten();
//            tokio::run(t);
//        }
//        Command::keys(KeysCmd::get(args)) => {
//            let t = task
//                .map(move |mut client| {
//                    client
//                        .get_key(args.name.as_str())
//                        .map_err(|err| eprintln!("request failed; err={:?}", err))
//                        .map(|resp| println!("{:?}", resp))
//                })
//                .flatten();
//            tokio::run(t);
//        }
//        Command::keys(KeysCmd::create(args)) => {
//            let key = make_key(args.name);
//            let t = task
//                .map(move |mut client| {
//                    client
//                        .create_key(key)
//                        .map_err(|err| eprintln!("request failed; err={:?}", err))
//                        .map(|resp| println!("{:?}", resp))
//                })
//                .flatten();
//            tokio::run(t);
//        }
//        Command::keys(KeysCmd::delete(args)) => {
//            let t = task
//                .map(move |mut client| {
//                    client
//                        .delete_key(args.name.as_str())
//                        .map_err(|err| eprintln!("request failed; err={:?}", err))
//                        .map(|resp| println!("{:?}", resp))
//                })
//                .flatten();
//            tokio::run(t);
//        }
    }
}

//

fn make_key(name: String) -> v1::Key {
    v1::Key {
        name: String::from(name),
        data: vec![],
        labels: HashMap::new(),
    }
}
