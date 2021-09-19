#[macro_use]
extern crate prost_derive;
#[macro_use]
extern crate serde;

// Include the `items` module, which is generated from items.proto.
include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));