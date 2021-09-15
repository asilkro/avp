use serde::{Serialize, Deserialize};
use crate::avp::Location;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Visited {
    Yes, No
}
// TODO: Constructors / Accessors for updating

pub fn visited(){
    return Visited(Location);
}

pub fn set_visited(){
    SetVisited{
        Visited.Add(Location);
    }
}