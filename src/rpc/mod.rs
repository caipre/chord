//! Traits and types for the Chord API

mod chord {
    pub mod v1 {
        use prost_derive::{Message, Enumeration};
        include!(concat!(env!("OUT_DIR"), "/chord.v1.rs"));
    }
}

pub use self::chord::v1;
