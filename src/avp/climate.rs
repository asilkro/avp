use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Climate {
    Cold, Hot, Moderate, Warm
}

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