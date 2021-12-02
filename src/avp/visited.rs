use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Visited {
    Yes, No
}