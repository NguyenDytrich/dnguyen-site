use rocket::{get, post};
use rocket_dyn_templates::Template;

use dnguyen_blog::htmlify::transcribe;

#[get("/editor")]
pub async fn console_index() -> Template {
    return Template::render("console/post_editor", {});
}

#[post("/preview", data="<content>")]
pub async fn gen_preview(content: String) -> String {
    transcribe(&content)
}
