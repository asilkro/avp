use serde::{Serialize, Deserialize};
use crate::avp::Location;
// use crate::avp::*;
    // Thought about doing this
use crate::avp::climate::Climate::{Cold, Hot, Moderate, Warm};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Climate {
    Cold, Hot, Moderate, Warm
}
// TODO: Constructors / Accessors for updating

    // Constructors & accessors defined in location.rs

// TODO: Make numbers be a thing that defines climate at some point
// impl Climate {
//     fn temperature(self) -> int {
//         match self {
//             Hot => (100 => && <= 90),
//             Warm => (89 => && <= 70),
//             Moderate => (69 => && <= 49),
//             Cold => (>=48),
//         }
//     }
// }