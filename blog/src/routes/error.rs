use rocket::catch;
use rocket_dyn_templates::Template;

use crate::routes::BaseContext;

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("error/404", BaseContext {
        title: "404",
        parent: "layout"
    })
}
