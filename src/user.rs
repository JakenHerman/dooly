use diesel::prelude::*;
use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use crate::db::DbPool;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct PublicUser {
    pub id: i32,
    pub username: String,
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user(pool: &State<DbPool>, new_user: Json<NewUser>) -> Result<&'static str, (Status, &'static str)> {
    if new_user.username.trim().is_empty() {
        return Err((Status::BadRequest, "Username cannot be empty"));
    }

    if new_user.password_hash.trim().is_empty() {
        return Err((Status::BadRequest, "Password cannot be empty"));
    }

    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;

    diesel::insert_into(users::table)
        .values(&new_user.into_inner())
        .execute(&mut connection)
        .map_err(|_| (Status::InternalServerError, "Failed to create user"))?;

    Ok("User created successfully!")
}

#[get("/users/<id>")]
pub fn get_user_by_id(pool: &State<DbPool>, id: i32) -> Result<Json<PublicUser>, (Status, &'static str)> {
    let mut connection = pool.get().map_err(|_| (Status::InternalServerError, "Failed to get connection from pool"))?;

    // Query only the id and username, not the password_hash
    let user = users::table
        .filter(users::id.eq(id))
        .select((users::id, users::username))  // Select only the fields you want
        .first::<PublicUser>(&mut connection)
        .map_err(|_| (Status::NotFound, "User not found"))?;

    Ok(Json(user))
}