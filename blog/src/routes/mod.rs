#[derive(rocket::serde::Serialize)]
struct BaseContext<'a> {
    title: &'a str,
    parent: &'a str,
}

#[derive(rocket::serde::Serialize)]
struct PaginatorPage<T> {
    index: i64,
    has_next: bool,
    next_index: i64,
    has_prev: bool,
    prev_index: i64,
    total_pages: i64,
    objects: Vec<T>
}

pub mod blog;
pub mod error;
