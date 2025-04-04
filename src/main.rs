#[macro_use]
extern crate rocket;

// Module declaration
#[allow(dead_code)]
mod auth;
#[allow(dead_code)]
mod core;
#[allow(dead_code)]
mod database;
#[allow(dead_code)]
mod role;
#[allow(dead_code)]
mod service;
#[allow(dead_code)]
mod user;

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
        .mount("/role", role::routes())
        .launch()
        .await?;

    Ok(())
}
