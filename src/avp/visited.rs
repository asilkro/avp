use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub enum Visited {
    Yes, No
}