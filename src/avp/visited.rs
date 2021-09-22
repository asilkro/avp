use serde::{Serialize, Deserialize};
use crate::avp::Location;
use std::fs::read;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Visited {
    Yes, No
}


// Do I even need these structs?
struct Yes {
    visited : yes
}

struct No {
    visited : no
}

// TODO: Constructors / Accessors for updating

// Accessor goes here
pub fn visited() {
    match visited_response() {
        Yes => Visited::Yes,
        No => Visited::No,
    };
}

pub fn set_visited() {

}