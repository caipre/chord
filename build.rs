extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["rpc/v1/chord.proto"], &["rpc/v1"]).unwrap();
}
