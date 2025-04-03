#[macro_use]
extern crate rocket;

// Module declaration
mod core;
mod database;
mod service;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Set up database connection pool
    let pool = database::open_or_create_db("db.sqlite")
        .await
        .expect("Couldn't open database");

    rocket::build()
        .manage(pool)
        .mount("/service", service::routes())
        .launch()
        .await?;

    Ok(())
}
