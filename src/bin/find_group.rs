use newsbase::*;

fn main() {
    let db_connection = establish_connection();

    let group = find_newsgroup(&db_connection, "alt.sysadmin.recovery");
    println!("Found {} ({}/{})", group.name, group.low, group.high);
}