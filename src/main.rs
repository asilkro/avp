mod avp;
mod error;

use avp::Locations;
use std::path::Path;
use error::{Error, Result};
use std::fs::{File};



fn main() -> Result<()> { //TODO Path should be replaced with a generic trait (Read)
    let config_filename = Path::new("./configfile.yml");
    let file = File::open(config_filename)?;

    let locs = Locations::new(file)?;
    dbg!(&locs);
    Ok(())
}
