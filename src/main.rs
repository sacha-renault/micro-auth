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
mod revoked_token;
#[allow(dead_code)]
mod role;
#[allow(dead_code)]
mod scope;
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
        .mount("/scope", scope::routes())
        .mount("/role", role::routes())
        .mount("/auth", auth::routes())
        .launch()
        .await?;

    Ok(())
}
