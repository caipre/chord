// fixme: should we be importing chord_rpc or should that be contained in chord?

/// todo:
///   - add flags for ip, port
///   - read key data from file
///   - box chord::grpc::client futures
///   - figure out server shutdown
///
use {
    chord::grpc::client, chord::grpc::server::ChordService, chord_rpc::v1::*, futures::prelude::*,
    prost_types::FieldMask, quicli::prelude::*, std::collections::HashMap, structopt::StructOpt,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "chord")]
struct ChordCli {
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
    enable,
    disable,
    stop,
}

#[derive(StructOpt, Debug)]
#[allow(non_camel_case_types)]
enum KeysCmd {
    list,
    get(GetKeyCmd),
    create(CreateKeyCmd),
    update(UpdateKeyCmd),
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
struct UpdateKeyCmd {
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

    let task = client::connect(
        &"[::1]:32031".parse().unwrap(),
        "http://localhost:32031".parse().unwrap(),
    );

    match chord.cmd {
        Command::node(NodeCmd::start) => {
            let srv = ChordService::new();
            srv.serve("[::1]:32031".parse().unwrap())
        }

        Command::node(NodeCmd::info) => {
            let t = task
                .map(move |mut client| {
                    client
                        .get_node()
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }

        Command::node(NodeCmd::enable) => {
            let node = make_node();
            let mask = make_mask();
            let t = task
                .map(move |mut client| {
                    client
                        .update_node(node, mask)
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }

        Command::node(NodeCmd::disable) => {
            let node = make_node();
            let mask = make_mask();
            let t = task
                .map(move |mut client| {
                    client
                        .update_node(node, mask)
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }

        Command::node(NodeCmd::stop) => {
            let node = make_node();
            let mask = make_mask();
            let t = task
                .map(move |mut client| {
                    client
                        .update_node(node, mask)
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }

        Command::keys(KeysCmd::list) => {
            let t = task
                .map(move |mut client| {
                    client
                        .list_keys()
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }
        Command::keys(KeysCmd::get(args)) => {
            let t = task
                .map(move |mut client| {
                    client
                        .get_key(args.name.as_str())
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }
        Command::keys(KeysCmd::create(args)) => {
            let key = make_key(args.name);
            let t = task
                .map(move |mut client| {
                    client
                        .create_key(key)
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }
        Command::keys(KeysCmd::update(args)) => {
            let key = make_key(args.name);
            let mask = make_mask();
            let t = task
                .map(move |mut client| {
                    client
                        .update_key(key, mask)
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }
        Command::keys(KeysCmd::delete(args)) => {
            let t = task
                .map(move |mut client| {
                    client
                        .delete_key(args.name.as_str())
                        .map_err(|err| eprintln!("request failed; err={:?}", err))
                        .map(|resp| println!("{:?}", resp))
                })
                .flatten();
            tokio::run(t);
        }
    }
}

//

fn make_node() -> Node {
    Node {
        name: String::from("node_name"),
        state: RunState::Starting.into(),
        predecessor: String::from("1.1.1.1"),
        routes: vec![],
        successors: vec![],
    }
}

fn make_key(name: String) -> Key {
    Key {
        name: String::from(name),
        data: vec![],
        labels: HashMap::new(),
    }
}

fn make_mask() -> FieldMask {
    FieldMask { paths: vec![] }
}
