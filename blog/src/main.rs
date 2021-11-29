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

    // Config checks
    let _stripe_public_key = env::var("STRIPE_PUBLIC_KEY").expect("Stripe public key not provided");
    let _stripe_secret_key = env::var("STRIPE_SECRET_KEY").expect("Stripe secret key not provided");

    let _server = rocket::build()
        .mount("/", routes![
                routes::blog::blog_index,
                routes::blog::blog_post,
                routes::blog::blog,
                routes::tipjar::thanks,
                routes::tipjar::post_payment
            ])
        .mount("/tipjar", routes![
               routes::tipjar::index,
            ])
        .mount("/api", routes![
                routes::tipjar::update_intent,
            ])
        .mount("/static", FileServer::from(static_dir))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
