use rocket::{get, post};
use rocket_dyn_templates::Template;

use dnguyen_blog::htmlify::transcribe;

#[get("/")]
pub async fn console_index() -> Template {
    return Template::render("console/console_index", {});
}

#[post("/preview", data="<content>")]
pub async fn gen_preview(content: String) -> String {
    transcribe(&content)
}
