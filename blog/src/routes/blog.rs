use rocket::{get, uri};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use crate::DbConn;
use crate::routes::{PaginatorPage, BaseContext};
use crate::models::{BlogPost, PublishState};
use crate::models::dto::BlogPostPreview;

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
pub async fn index(db_conn: DbConn) -> Result<Template, Status> {
    page(db_conn, 1).await
}

#[get("/?<page>")]
pub async fn page(db_conn: DbConn, page: i64) -> Result<Template, Status> {

    use crate::schema::blog_posts::dsl::*;
    use crate::diesel::query_dsl::*;
    use crate::diesel::dsl::count_star;
    use rocket_sync_db_pools::diesel::RunQueryDsl;

    if page <= 0 {
        return Err(Status::NotFound)
    }

    // TODO (Dytrich Nguyen): make configurable
    let paginate_by = 5;

    // Get count of all posts
    let count: i64 = db_conn.run(move |c| {
        blog_posts.select(count_star()).first(c)
    }).await.unwrap_or(-1);

    if page > count {
        return Err(Status::NotFound)
    }

    // Select a number of posts
    let posts: Vec<BlogPost> = db_conn.run(move |c| {
        blog_posts
            .offset(paginate_by * (page - 1))
            .limit(5)
            .load(c)
    }).await.unwrap_or(Vec::new());

    let previews: Vec<BlogPostPreview> = posts.iter()
        .map(|p| BlogPostPreview::from(p)).collect();

    let pages = count as f64 / paginate_by as f64;
    let pages = pages.ceil() as i64;
    let pages = if pages > 0 {pages} else {1};
    let next = pages > page;
    let prev = page > 1;

    Ok(Template::render(
        "blog/index", BlogContext {
            title: "Blog",
            parent: "layout",
            paginator: PaginatorPage {
                index: page,
                has_next: next,
                next_index: page + 1,
                has_prev: prev,
                prev_index: page - 1,
                total_pages: pages,
                objects: previews
            }
        }))
}

// TODO (Dytrich Nguyen):
// Maybe there's a way to redirect to the correct slug if the ID is right but slug isnt?
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

// TODO (Dytrich Nguyen):
// Maybe write a fairing to carry a blog_post through to the post route?
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
