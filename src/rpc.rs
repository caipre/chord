pub mod google {
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
    pub mod rpc {
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
    }
}
pub mod chord {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/chord.v1.rs"));
    }
}