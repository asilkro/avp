use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub enum Climate {
    Cold, Hot, Moderate, Warm
}