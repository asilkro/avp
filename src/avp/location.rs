use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::{avp::climate::Climate, Result};
use crate::avp::visited::Visited;

#[cfg(test)]
mod unit_tests;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    climate: Climate,
    distance: u32,
    visited: Visited,
}

impl Location {
    pub fn climate(&self) -> &Climate {
        &self.climate
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }
    pub fn visited(&self) -> &Visited {
        &self.visited
    }
}

//YAGNI - You ain't gonna need it
//DRY - Don't Repeat Yourself

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Locations {
    locations: Vec<Location>,
}

impl Locations {
    pub fn new<TRead>(mut reader: TRead) -> Result<Self> where TRead: Read {
        // Read data into memory (allows us to check if reader is empty before
        // we try to deserialize it into a `Locations` instance.)
        let data = {
            let mut tmp = Vec::new();
            reader.read_to_end(&mut tmp)?;
            tmp
        };
        // Check if we have any data:
        let locs = match data.len() {
            // No data read; `reader` was empty, so return an empty `Locations`.
            // Note `serde_yaml` would have errored in this situation, so by
            // pre-screening for it, we can ensure when `serde_yaml` returns an
            // error, it's not because the `Reader` was empty.
            0 => Locations { locations: Vec::new() },
            // Data was read.  Attempt to deserialize it as a `Locations`
            // instance.  If `serde_yaml` errors, then we know it was an error
            // we consider legitimate, so return it as such.
            _ => serde_yaml::from_slice(data.as_ref())?,
        };
        Ok(locs)
    }

    // TODO: I want a way to open a thing; we already have new ^^ above but I want something else to implement read.
    //  The thing I'm making will go below here.
    //
    // pub fn open()
    // }



    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.locations.is_empty()
        // locations is our data store so if it's empty, we know it has to be empty
        // the vec of Location already implements is_empty so we can reuse
    }
}

//TODO: Constructors / Accessors for updating