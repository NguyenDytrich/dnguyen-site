use rocket::get;
use rocket_dyn_templates::Template;

use crate::routes::{BaseContext, PaginatorPage};

#[derive(rocket::serde::Serialize)]
struct PostPreview;

// TODO: modularize Contexts
#[derive(rocket::serde::Serialize)]
struct BlogContext<'a> {
    title: &'a str,
    parent: &'a str,
    paginator: PaginatorPage<PostPreview>
}

#[get("/")]
pub async fn index() -> Template {
    // TODO: retrieve blog posts
    let count = 0;

    if count == 0 {
        return Template::render("blog/no_posts", BaseContext {
            title: "Blog",
            parent: "layout"
        });
    }

    let page = PaginatorPage {
        index: -1,
        next_page: -1,
        prev_page: -1,
        total_pages: 0,
        objects: vec![]
   };

    Template::render("blog/index", BlogContext {
        title: "Blog",
        parent: "layout",
        paginator: page
    })
}
