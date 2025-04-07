#[macro_use]
extern crate rocket;

// Module declaration
mod auth;
mod core;
mod database;
mod revoked_token;
mod role;
mod scope;
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
        .mount("/scope", scope::routes())
        .mount("/role", role::routes())
        .mount("/auth", auth::routes())
        .launch()
        .await?;

    Ok(())
}
