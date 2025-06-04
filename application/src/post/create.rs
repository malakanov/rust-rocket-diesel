use domain::models::{NewPost};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_post(post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;

    let post = post.into_inner();

    let mut conn = establish_connection();

    diesel::insert_into(posts::table).values(&post).execute(&mut conn).unwrap();

    let inserted_post = posts::table.order(posts::id.desc()).first(&mut conn).unwrap();

    let response = Response { body: ResponseBody::Post(inserted_post) };
    Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
}