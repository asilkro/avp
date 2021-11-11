mod avp;
mod error;

use avp::Locations;
use std::path::Path;
use error::Result;
use std::fs::{File};

/// Requirements:
/// * launch `avp`
/// * Prompt for desired climate (Hot, Warm, Moderate or Cold)
/// * Prompt for max distance willing to travel (miles)
/// * Prompt for preference: new or previously visited destination
/// * `avp` responds:
/// No options: "Sorry, I can't find anything that matches."
///
/// 1 option: "Ok, here is the option" with the destination matching above criteria
///
/// 2+ options: "Ok, here are some options" with up to 3 options matching above criteria ordered from nearest to furthest
///
/// * search again or quit


fn main() -> Result<()> { //TODO Path should be replaced with a generic trait (Read)
    let config_filename = Path::new("./configfile.yml");
    let file = File::open(config_filename)?;

    let locs = Locations::new(file)?;
    dbg!(&locs);
    Ok(())
}
