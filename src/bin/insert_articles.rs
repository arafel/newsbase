use nntp::NNTPStream;
use substring::Substring;
use std::collections::HashMap;

use newsbase::*;

fn main() {
    let db_connection = establish_connection();

    let mut nntp_stream = match NNTPStream::connect(("nntp.aioe.org", 119)) {
        Ok(stream) => stream,
        Err(e) => std::panic::panic_any(e),
    };

    let group = find_newsgroup(&db_connection, "alt.sysadmin.recovery");
    println!("Found {} ({}/{})", group.name, group.low, group.high);

    match nntp_stream.group(&group.name.to_string()) {
        Ok(_) => println!("Selected group"),
        Err(e) => std::panic::panic_any(e),
    }

    for art_num in (group.low..group.high).take(5) {
        let res =
            match nntp_stream.head_by_number(art_num as isize) {
                Ok(res) => res,
                Err(e) => std::panic::panic_any(e)
            };

        let new_headers = extract_headers(&res);
        let article = create_article(&db_connection,
                                     &art_num,
                                     new_headers["From"],
                                     &group.id,
                                     new_headers["Subject"],
                                     new_headers["Date"]);
        println!("Article {:?}", article);
    }

    let _ = nntp_stream.quit();
}

fn extract_headers(headers: &Vec<String>) -> HashMap<&str, &str> {
    let interesting_headers = ["From", "Subject", "Date"];
    let mut new_headers = HashMap::new();

    for x in headers {
        let index =
            match x.find(":") {
                Some(res) => res,
                None => continue
            };
        let (key, value) = x.split_at(index);
        if interesting_headers.contains(&key) {
            new_headers.insert(key, value.substring(2, value.len() - 2));
        }
    }
    new_headers
}