mod chord {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/chord.v1.rs"));
    }
}

pub use self::chord::v1;
