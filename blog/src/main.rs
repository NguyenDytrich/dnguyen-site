use rocket::{routes, catch, catchers};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

use dotenv::dotenv;

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

    let _server = rocket::build()
        .mount("/", routes![
                routes::blog::blog_index,
                routes::blog::blog_post,
                routes::blog::blog,
            ])
        .mount("/static", FileServer::from(relative!("/static")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
