#[derive(rocket::serde::Serialize)]
struct BaseContext<'a> {
    title: &'a str,
    parent: &'a str,
}

#[derive(rocket::serde::Serialize)]
struct PaginatorPage<T> {
    index: isize,
    next_page: isize,
    prev_page: isize,
    total_pages: usize,
    objects: Vec<T>
}

pub mod blog;
pub mod error;
