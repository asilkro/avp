mod climate;
mod location;
#[cfg(test)]
mod unit_tests;

use crate::{error::io, Result};
use std::path::Path;
use std::fs::File;
use climate::Climate;
use serde::Serialize;
use serde_yaml;
use crate::error::Error;
pub use location::{Location,Locations};

