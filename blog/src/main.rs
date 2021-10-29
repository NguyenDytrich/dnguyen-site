use rocket::{routes, catch, catchers};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

use dotenv::dotenv;
use std::env;

mod routes;

#[catch(404)]
fn not_found() -> Template {
    Template::render("error/404", context! {
        title: "404",
        parent: "layout"
    })
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    let static_dir = &env::var("STATIC_DIR").unwrap_or(relative!("/static").to_string());

    let _server = rocket::build()
        .mount("/", routes![
                routes::blog::blog_index,
                routes::blog::blog_post,
                routes::blog::blog,
            ])
        .mount("/console", routes![routes::console::console_index])
        .mount("/api", routes![routes::console::gen_preview])
        .mount("/static", FileServer::from(static_dir))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
