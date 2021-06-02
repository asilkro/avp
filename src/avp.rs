#[cfg(test)]
mod unit_tests;

use crate::{error::io, Result};
use std::path::Path;
use std::fs::File;

#[derive(Debug)]
pub struct Avp {}

impl Avp {
    pub fn new(path: &Path) -> Result<Self, io::Error> {
        let _file = File::open(path)?;
        Ok(Avp {})
    }

    pub fn run(self) -> Result<()> {
        todo!()
    }
}