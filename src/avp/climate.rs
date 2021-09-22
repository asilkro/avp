use serde::{Serialize, Deserialize};
use crate::avp::Location;
use crate::avp::climate::Climate::Moderate;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Climate {
    Cold, Hot, Moderate, Warm
}
// TODO: Constructors / Accessors for updating

/*
// TODO: Make numbers be a thing that defines climate at some point
impl Climate {
    fn temperature(self) -> int {
        match self {
            Hot => (100 => && <= 90),
            Warm => (89 => && <= 70),
            Moderate => (69 => && <= 49),
            Cold => (>=48),
        }
    }
}
*/

struct Hot {
    climate : hot
}

struct Warm {
    climate : warm
}

struct Moderate {
    climate : moderate
}

struct Cold {
    climate : cold
}

pub fn climate() {
    match climate_type() {
        Cold => Climate::Cold,
        Moderate => Climate::Moderate,
        Warm => Climate::Warm,
        Hot => Climate::Hot,
        _ => println!("Error, choose Cold, Moderate, Warm or Hot"),
    };
}