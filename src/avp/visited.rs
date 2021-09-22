use serde::{Serialize, Deserialize};
use crate::avp::Location;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Visited {
    Yes, No
}
// TODO: Constructors / Accessors for updating

struct Yes {
    visited : yes
}

struct No {
    visited : no
}

// First shot an
pub fn visited() {
    match visited_response() {
        Yes => Visited::Yes,
        No => Visited::No,
        _ => println!("Error, choose Yes or No"),
    };
}