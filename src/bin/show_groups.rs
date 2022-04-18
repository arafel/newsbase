extern crate diesel;

use newsbase::*;
use newsbase::models::*;

use self::diesel::prelude::*;

fn main() {
    use newsbase::schema::newsgroups::dsl::*;

    let connection = establish_connection();
    let results = newsgroups.filter(low.eq(0))
        .limit(5)
        .load::<Newsgroup>(&connection)
        .expect("Error loading newsgroups");

    println!("Displaying {} newsgroups", results.len());
    for newsgroup in results {
        println!("{}", newsgroup.name);
        println!("----------\n");
        println!("{}", newsgroup.low);
        println!("{}", newsgroup.high);
    }
}