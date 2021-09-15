use serde::{Serialize, Deserialize};
use crate::avp::Location;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Climate {
    Cold, Hot, Moderate, Warm
}
// TODO: Constructors / Accessors

pub fn get {
    return Climate(Location);
}

pub fn set {
    SetClimate{
        climate.Add(Location);
    }
}
