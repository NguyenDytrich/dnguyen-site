#[macro_use]
extern crate diesel;

mod routes;
mod schema;
mod models;
mod utils;

use std::env;
use std::path::Path;

use rocket::{routes, catch, catchers, figment};
use rocket::fs::{FileServer, relative};
use rocket::serde::{Serialize, Deserialize};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database};

use routes::{blog, error};

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[rocket::main]
async fn main() {

    // Custom config
    #[derive(Deserialize)]
    struct Config<'a> {
        static_dir: Option<&'a str>,
    }

    let figment = rocket::Config::figment();
    let config: Config = figment.extract().expect("Error extracting config");

    // Get the configured static directory
    // otherwise just use the one in current directory
    let static_dir = match config.static_dir {
        Some(val) => Path::new(val),
        None => Path::new(relative!("static"))
    };

    let _server = rocket::custom(figment)
        .mount("/static", FileServer::from(static_dir))
        .mount("/", routes![
               blog::index,
        ])
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .register("/", catchers![
              error::not_found
        ])
        .launch()
        .await;
}
