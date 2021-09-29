use serde::{Serialize, Deserialize};
use crate::avp::Location;
use std::fs::read;
use crate::avp::visited::Visited::Yes;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Visited {
    Yes, No
}


// TODO: Constructors / Accessors for updating

// Accessor goes here

