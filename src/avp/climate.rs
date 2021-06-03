use serde::Serialize;
#[derive(Debug, Serialize)]
pub enum Climate {
    Cold, Hot, Moderate, Warm
}