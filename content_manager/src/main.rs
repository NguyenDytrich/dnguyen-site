#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use iced::{Application, Button, Clipboard, Column, Command, Element, Scrollable, Settings, Text};
use iced::{executor, scrollable};
use models::BlogPost;
use std::env;

fn main() -> iced::Result {
    App::run(Settings::default())
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    use diesel::prelude::*;
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}

#[derive(Debug)]
enum App {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    posts: Vec<BlogPost>,
}

#[derive(Debug)]
enum StateError {
    LoadingError
}


#[derive(Debug)]
enum Message {
    Loaded(Result<State, StateError>),
    Dummy
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) { 
        use diesel::prelude::*;
        use self::schema::blog_posts::dsl::*;

        let connection = establish_connection();
        let results = blog_posts.load::<BlogPost>(&connection)
            .map(|posts| State { posts })
            .map_err(|_| StateError::LoadingError);

        match results {
            Ok(state) => (Self::Loaded(state), Command::none()),
            Err(_) => (Self::Loading, Command::none())
        }
    }

    fn title(&self) -> String {
        String::from("DNguyen CMS")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            App::Loaded(State {
                posts,
                ..
            }) => {
                posts
                    .iter()
                    .fold(Column::new().spacing(20), |column, val| {
                        column.push(
                            Text::new(String::from(&val.title))
                        )
                    }).into()
            }
            _ => Text::new("Test").into()
        }
    }
}
