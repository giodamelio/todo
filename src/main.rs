#[macro_use]
extern crate diesel;

mod args;
mod models;
mod schema;

use std::env;

use anyhow::{Context, Result};
use clap::crate_version;
use log::{debug, error, info};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn database_connect() -> Result<SqliteConnection> {
    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;
    SqliteConnection::establish(&database_url).context("Could not connect to database")
}

fn go() -> Result<()> {
    let args = args::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(args.log_level)
        .init();

    debug!("TODO version {}", crate_version!());
    info!("Args: {:?}", args);

    // Connect to the database
    let connection = database_connect()?;

    // List the todos
    use schema::todo::dsl::*;

    let todo_list = todo.load::<models::Todo>(&connection)?;
    println!("Listing todos:");
    for todo_item in todo_list {
        println!("{:?}", todo_item);
    }
    Ok(())
}

fn main() {
    match go() {
        Err(error) => {
            error!("Error: {}", error);
            for err in error.chain().skip(1) {
                error!("    {}", err)
            }
        }
        Ok(()) => {}
    }
}
