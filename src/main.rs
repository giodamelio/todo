#[macro_use]
extern crate diesel;

mod args;
mod logging;
mod models;
mod schema;

use std::env;

use anyhow::{Context, Result};
use clap::crate_version;
use log::{debug, error, info, trace};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use logging::Logger;

fn database_connect() -> Result<SqliteConnection> {
    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;
    SqliteConnection::establish(&database_url).context("Could not connect to database")
}

fn go() -> Result<()> {
    // Start up the logger
    let logger = Logger::init()?;

    // Parse the args
    let args = args::parse();
    debug!("Cli arguments: {:?}", args);

    // Setup the logging with the args
    logger.set_from_args(&args);

    // Say hello!
    debug!("TODO version {}", crate_version!());

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
