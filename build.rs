extern crate tower_grpc_build;

fn main() {
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/v1/chord.proto"], &["proto/v1"])
        .expect("protobuf compilation failed");
}
