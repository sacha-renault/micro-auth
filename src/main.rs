#[macro_use]
extern crate rocket;

// Module declaration
mod core;
mod database;
mod service;

use service::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Set up database connection pool
    let pool = database::open_or_create_db("db.sqlite")
        .await
        .expect("Couldn't open database");

    rocket::build()
        .manage(pool)
        .mount("/service", routes![add_service])
        .launch()
        .await?;

    Ok(())
}
