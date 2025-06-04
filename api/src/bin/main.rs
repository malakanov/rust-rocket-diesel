#[macro_use] extern crate rocket;
use api::post_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 4000)))
        .mount("/api", routes![
            post_handler::list_posts_handler, 
            post_handler::list_post_handler,
            post_handler::create_post_handler,
        ])
}