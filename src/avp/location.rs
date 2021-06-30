use crate::{avp::climate::Climate, Result};
use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    climate: Climate,
    distance: u32,
}

//YAGNI - You ain't gonna need it
//DRY - Don't Repeat Yourself

#[derive(Debug, Serialize, Deserialize,)]
pub struct Locations {
    locations: Vec<Location>,
}

impl Locations {
    pub fn new<TReader: Read>(reader: TReader) -> Result<Self> {
        // let locs = serde_yaml::from_reader(reader)?;
        let locs = match reader.metadata()?.len() {
            0 => Self { locations: Vec::new() }, // if 0 do this
            _ => serde_yaml::from_reader(reader)? // else, do that
        };
        Ok(locs)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.locations.len()
    }

}