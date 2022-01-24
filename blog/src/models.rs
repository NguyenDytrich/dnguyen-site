use std::convert::From;
use std::io::Write;

use chrono::prelude::*;
use serde::Serialize;

use crate::utils::htmlify::{monthify, transcribe};

pub enum PublishState {
    Draft,
    Public,
    Unlisted,
    Archived,
}

// TODO:
// update to support multiple body formats
// ex. HTML

#[derive(Queryable, Serialize)]
pub struct BlogPost {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub publish_state: String,
    pub body: Option<String>,
    pub title: String,
    pub slug: String,
}

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

impl BlogPost {
    fn content_preview(&self) -> Option<String> {

        // Return if nothing to preview
        let body = match &self.body {
            Some(value) => value,
            None => return None
        };

        if body.len() <= 300 {
            return Some(body.to_owned())
        }

        // Create a new string buffer
        let mut preview = String::new();
        // Possible end of word chars
        let c_eow = [' ', '\n', '\r'];

        // Index of end of word. Start search from 255th char
        let mut i_eow = 255;
        let mut c: char = 'a'; // arbitrarily assign a char to start

        while !c_eow.contains(&c) {
            // Unwrap otherwise stop the search if index is out of bounds
            c = body.chars().nth(i_eow).unwrap_or(' ');
            i_eow = i_eow + 1;
        }

        // Copy the string to the buffer
        preview.push_str(&body[0..(i_eow - 1)]);
        preview.push_str("...");
        Some(preview)
    }
}

