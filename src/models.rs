use super::schema::newsgroups;

#[derive(Insertable)]
#[table_name="newsgroups"]
pub struct NewNewsgroup<'a> {
    pub name: &'a str,
    pub low: &'a i32,
    pub high: &'a i32,
}
#[derive(Queryable)]
pub struct Newsgroup {
    pub id: i32,
    pub name: String,
    pub low: i32,
    pub high: i32
}