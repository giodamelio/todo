mod args;

use clap::crate_version;
use log::debug;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(args.log_level)
        .init();

    debug!("TODO version {}", crate_version!());
    println!("Args: {:?}", args);

    Ok(())
}
