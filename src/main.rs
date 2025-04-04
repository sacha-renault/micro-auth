#[macro_use]
extern crate rocket;

// Module declaration
mod core;
mod database;
mod role;
mod service;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Set up database connection pool
    let pool = database::open_or_create_db("db.sqlite")
        .await
        .expect("Couldn't open database");
    database::create_tables(&pool)
        .await
        .expect("Couldn't create table");

    rocket::build()
        .manage(pool)
        .mount("/service", service::routes())
        .launch()
        .await?;

    Ok(())
}
