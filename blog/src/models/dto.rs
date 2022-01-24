use std::convert::From;

use chrono::prelude::*;

use serde::Serialize;

use super::BlogPost;
use crate::utils::htmlify::{monthify, transcribe};

#[derive(Serialize)]
pub struct BlogPostPreview {
    pub id: i32,
    pub title: String,
    pub date_repr: String,
    pub preview: String,
    pub slug: String,
}


impl From<&BlogPost> for BlogPostPreview {
    fn from(blog_post: &BlogPost) -> Self {

        let date = match blog_post.published_at {
            Some(d) => (d.day(), d.month(), d.year()),
            None => (
                blog_post.created_at.day(),
                blog_post.created_at.month(),
                blog_post.created_at.year())
        };

        BlogPostPreview {
            id: blog_post.id,
            title: blog_post.title.clone(),
            slug: blog_post.slug.clone(),
            date_repr: format!("{:02}, {} {}",
                    date.0,
                    monthify(date.1 as usize).unwrap_or("ERR".to_string()),
                    date.2),
            preview: transcribe(
                &blog_post.content_preview()
                    .unwrap_or(String::new())),
        }
    }
}
