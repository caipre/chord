//! Traits and types for the Chord API

pub use self::chord::v1;

#[allow(dead_code)]
mod chord {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/chord.v1.rs"));
    }
}

