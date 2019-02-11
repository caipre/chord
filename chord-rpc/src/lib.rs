//! Traits and types for the Chord API

#[allow(dead_code)]
mod chord {
    pub mod v1 {
        use prost_derive::{Enumeration, Message};
        include!(concat!(env!("OUT_DIR"), "/chord.v1.rs"));
    }
}

pub use self::chord::v1;
