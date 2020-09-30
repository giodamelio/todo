#[macro_use]
extern crate diesel;

mod args;
mod models;
mod schema;

use std::env;

use anyhow::{Context, Result};
use clap::crate_version;
use log::{debug, error, info, trace};
use flexi_logger::{Logger, LogSpecBuilder};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn database_connect() -> Result<SqliteConnection> {
    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;
    SqliteConnection::establish(&database_url).context("Could not connect to database")
}

fn go() -> Result<()> {
    // Start the default logger
    let mut reconfig_logger = Logger::with_env()
        .start()
        .context("Logger initialization failed")?;

    // Parse the args
    let args = args::parse();

    // Update the logger level if the cli flag specifies it
    reconfig_logger.set_new_spec(
        LogSpecBuilder::new()
            .default(args.log_level)
            .build(),
    );

    debug!("TODO version {}", crate_version!());
    info!("Args: {:?}", args);

    // Connect to the database
    let connection = database_connect()?;

    // List the todos
    use schema::todo::dsl::*;

    let todo_list = todo.load::<models::Todo>(&connection)?;
    info!("Listing todos:");
    for todo_item in todo_list {
        info!("{:?}", todo_item);
    }
    Ok(())
}

fn main() {
    match go() {
        Err(error) => {
            error!("Error: {}", error);
            trace!("Error chain:");
            for err in error.chain() {
                trace!(" - {}", err)
            }
        }
        Ok(()) => {}
    }
}
