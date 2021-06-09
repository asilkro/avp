use crate::{avp::climate::Climate, Result};
use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    climate: Climate,
    distance: u32,
}

//YAGNI - You ain't gonna need it
//DRY - Don't Repeat Yourself

#[derive(Debug, Serialize, Deserialize)]
pub struct Locations {
    locations: Vec<Location>,
}

impl Locations {
    pub fn new(path: &Path) -> Result<Self> {
        let reader = File::open(path)?;
        let locs = serde_yaml::from_reader(reader)?;
        Ok(locs)
    }
}