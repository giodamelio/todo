mod args;

use clap::crate_version;
use log::{debug, error, info, trace, warn};

fn main() {
    pretty_env_logger::init();

    debug!("TODO version {}", crate_version!());

    let args = args::parse();

    println!("Args: {:?}", args);
}
