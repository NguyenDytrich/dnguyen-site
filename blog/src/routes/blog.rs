use rocket::get;
use rocket_dyn_templates::Template;

use crate::DbConn;
use crate::routes::{PaginatorPage, BaseContext};
use crate::models::{BlogPost, BlogPostPreview};

// TODO: modularize Contexts
#[derive(rocket::serde::Serialize)]
struct BlogContext<'a> {
    title: &'a str,
    parent: &'a str,
    paginator: PaginatorPage<BlogPostPreview>
}

#[get("/")]
pub async fn index(db_conn: DbConn) -> Template {

    use crate::schema::blog_posts::dsl::*;
    use rocket_sync_db_pools::diesel::RunQueryDsl;

    let posts: Vec<BlogPost> = db_conn.run(|c| {
        blog_posts.load(c)
    }).await.unwrap_or(Vec::new());

    if posts.len() == 0 {
        return Template::render("blog/no_posts", BaseContext {
            title: "Blog",
            parent: "layout"
        });
    }

    let previews: Vec<BlogPostPreview> = posts.iter()
        .map(|p| BlogPostPreview::from(p)).collect();

    let page = PaginatorPage {
        index: -1,
        next_page: -1,
        prev_page: -1,
        total_pages: 0,
        objects: previews
   };

    Template::render("blog/index", BlogContext {
        title: "Blog",
        parent: "layout",
        paginator: page
    })
}
