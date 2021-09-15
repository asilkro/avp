use serde::{Serialize, Deserialize};
use crate::avp::Location;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Visited {
    Yes, No
}
// TODO: Constructors / Accessors

pub fn get {
    return Visited(Location);
}

pub fn set {
    SetVisited{
        Visited.Add(Location);
    }
}