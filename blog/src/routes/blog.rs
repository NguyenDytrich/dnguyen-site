use rocket::{get, uri};
use rocket::http::Status;
use rocket::response::Redirect;
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

#[derive(rocket::serde::Serialize)]
struct PostContext<'a> {
    title: &'a str,
    parent: &'a str,
    content: String
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

#[get("/<post_id>/<_slug>")]
pub async fn post(db_conn: DbConn, post_id: i32, _slug: String) -> Result<Template, Status> {

    use crate::schema::blog_posts::dsl::*;
    use crate::diesel::query_dsl::*;
    use rocket_sync_db_pools::diesel::RunQueryDsl;

    let result: Result<BlogPost, diesel::result::Error> = db_conn.run(move |c| {
        blog_posts.find(post_id).first(c)
    }).await;

    match result {
        Ok(post) => Ok(Template::render(
            "blog/post", PostContext {
                title: &post.title,
                parent: "layout",
                content: post.render_html()
            })),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/<id_or_slug>")]
pub async fn redirect_from_id_or_slug(db_conn: DbConn, id_or_slug: String) -> Result<Redirect, Status> {

    use crate::schema::blog_posts::dsl::*;
    use crate::diesel::query_dsl::*;
    use crate::diesel::expression_methods::*;
    use rocket_sync_db_pools::diesel::RunQueryDsl;

    let result: Result<(i32, String), diesel::result::Error> = match id_or_slug.parse::<i32>() {
        Ok(i) => db_conn.run(move |c| {
                blog_posts.find(i).select((id, slug)).first(c)
            }).await,
        // Could not parse int, so maybe it's a slug:
        Err(_) => db_conn.run(|c| {
                blog_posts.filter(slug.eq(id_or_slug)).select((id, slug)).first(c)
            }).await
    };

    match result {
        Ok(v) => Ok(Redirect::to(uri!(post(v.0, v.1)))),
        Err(_) => Err(Status::NotFound)
    }
}
