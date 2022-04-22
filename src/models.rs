use super::schema::newsgroups;
use super::schema::articles;

#[derive(Insertable)]
#[table_name="newsgroups"]
pub struct NewNewsgroup<'a> {
    pub name: &'a str,
    pub low: &'a i32,
    pub high: &'a i32,
}
#[derive(Queryable, Debug)]
pub struct Newsgroup {
    pub id: i32,
    pub name: String,
    pub low: i32,
    pub high: i32
}

#[derive(Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub newsgroup_id: i32,
    pub server_id: i32,
    pub author: String,
    pub subject: String,
    pub date_sent: String
}
#[derive(Insertable)]
#[table_name="articles"]
pub struct NewArticle<'a> {
    pub newsgroup_id: &'a i32,
    pub server_id: &'a i32,
    pub author: &'a str,
    pub subject: &'a str,
    pub date_sent: &'a str
}
