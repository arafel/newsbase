#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::{NewNewsgroup, Newsgroup};
use crate::models::{NewArticle, Article};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_newsgroup<'a>(conn: &PgConnection, name: &'a str, low: &'a i32, high: &'a i32, last_high: &'a i32) -> Newsgroup {
    use schema::newsgroups;

    let new_newsgroup = NewNewsgroup {
        name,
        low,
        high,
        last_high
    };

    diesel::insert_into(newsgroups::table)
        .values(&new_newsgroup)
        .get_result(conn)
        .expect("Error saving new newsgroup")
}

pub fn create_article<'a>(conn: &PgConnection,
                          server_id: &'a i32,
                          author: &'a str,
                          newsgroup_id: &'a i32,
                          subject: &'a str,
                          date_sent: &'a str) -> Article {
    use schema::articles;

    let new_article = NewArticle {
        newsgroup_id,
        server_id,
        author,
        subject,
        date_sent
    };

    diesel::insert_into(articles::table)
        .values(&new_article)
        .get_result(conn)
        .expect("Error saving new article")
}

pub fn find_newsgroup(conn: &PgConnection, search_name: &str) -> Newsgroup {
    use schema::newsgroups::dsl::*;

    println!("Looking for {}", search_name);
    newsgroups.filter(name.like(search_name))
        .first(conn)
        .expect("No newsgroup found")
}