#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests;

mod diesel_sqlite;
mod rusqlite;
mod sqlx;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(sqlx::stage())
        .attach(rusqlite::stage())
        //
        //
        //
        .attach(diesel_sqlite::stage())
}
