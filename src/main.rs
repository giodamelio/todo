#[macro_use]
extern crate diesel;

mod args;
mod models;
mod schema;

use clap::crate_version;
use log::debug;
use std::env;
use std::error::Error;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn database_connect() -> Result<SqliteConnection, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url).expect("Error connecting"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(args.log_level)
        .init();

    debug!("TODO version {}", crate_version!());
    println!("Args: {:?}", args);

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
