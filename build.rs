fn main() {
//    prost_build::compile_protos(&["proto/v1/chord.proto"], &["proto/v1"]).unwrap();
    tower_grpc_build::Config::new()
        .build(&["proto/v1/chord.proto"], &["proto/v1"])
        .unwrap_or_else(|e| panic!("proto compilation failed: {}", e));
}
