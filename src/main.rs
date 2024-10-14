#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

#[get("/todos")]
fn get_todos() -> Json<Vec<TodoItem>> {
    let todos = vec![
        TodoItem { id: 1, title: "Learn Rust".to_string(), completed: false },
        TodoItem { id: 2, title: "Build a REST API".to_string(), completed: false },
    ];
    Json(todos)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_todos])
}