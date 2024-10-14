#[macro_use] extern crate rocket;

mod db;
mod schema;
mod routes;

#[launch]
fn rocket() -> _ {
    let pool = db::establish_connection();
    rocket::build()
        .manage(pool)
        .mount("/", routes![routes::get_todos, routes::add_todo, routes::delete_todo, routes::update_todo, routes::complete_todo])
}
