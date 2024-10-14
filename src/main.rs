#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use log::info;
use env_logger;
use std::io::Write;

mod db;
mod schema;
mod routes;

#[launch]
fn rocket() -> _ {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let timestamp = buf.timestamp();  // Get the timestamp
            write!(buf, "[{}] - {} - {}\n", timestamp, record.level(), record.args())  // Correct use of write!
        })
        .init();


    info!("Starting Dooly API server");
    
    let pool = db::establish_connection();
    info!("Database pool established");

    rocket::build()
        .attach(AdHoc::on_liftoff("Logger", |_| Box::pin(async move {
            info!("Rocket has launched successfully!");
        })))
        .manage(pool)
        .mount("/", routes![routes::get_todos, routes::add_todo, routes::delete_todo, routes::update_todo, routes::complete_todo])
}
