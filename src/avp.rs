mod climate;
#[cfg(test)]
mod unit_tests;

use crate::{error::io, Result};
use std::path::Path;
use std::fs::File;
use climate::Climate;
use serde::Serialize;
use serde_yaml;
use crate::error::Error;

#[derive(Debug, Serialize)]
pub struct Location {
    climate: Climate,
    distance: u32,
}

impl Location {
    pub fn new(path: &Path) -> Result<Self> {
        let locations = vec![Location {climate:Climate::Moderate, distance:500}];
        let db = File::create(path)?;
        serde_yaml::to_writer(db, &locations)?;
        // let _file = File::open(path)?;
        Ok(Location {climate:Climate::Moderate, distance:500})
    }

    pub fn run(self) -> Result<()> {
        todo!()
    }
}