use serde::{Serialize, Deserialize};
use crate::avp::Location;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Climate {
    Cold, Hot, Moderate, Warm
}
// TODO: Constructors / Accessors for updating


// pub fn set_climate(){
//     SetClimate{
//         climate.Add(Location);
//     }
// }
pub fn climate(&self) -> &Climate {
    &self.climate
}
