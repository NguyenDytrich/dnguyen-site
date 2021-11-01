use rocket::get;
use rocket_dyn_templates::Template;

#[get("/")]
pub async fn index() -> Template {
    Template::render(
        "tipjar/index", context! {
            parent: "layout",
        }
    )
}

