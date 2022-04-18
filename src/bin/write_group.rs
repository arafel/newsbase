extern crate newsbase;
extern crate diesel;

use newsbase::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like your name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    let low = 0;
    let high = 100;

    let group = create_newsgroup(&connection, name, &low, &high);
    println!("\nSaved newsgroup {} with id {}", name, group.id);
}
