use chrono::prelude::*;
use serde::Serialize;

use crate::utils::htmlify::transcribe;

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

impl BlogPost {
    pub fn content_preview(&self) -> Option<String> {

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

    pub fn render_html(&self) -> String {
        transcribe(self.body.as_ref().unwrap_or(&String::new()))
    }
}

