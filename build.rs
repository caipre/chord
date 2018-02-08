extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["rpc/v1/chord.proto"], &["/usr/local/include", "/usr/include", "rpc/v1"])
        .unwrap();
}
