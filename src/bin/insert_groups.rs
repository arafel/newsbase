use nntp::NNTPStream;

use newsbase::*;

fn main() {
    let db_connection = establish_connection();

    let mut nntp_stream = match NNTPStream::connect(("nntp.aioe.org", 119)) {
        Ok(stream) => stream,
        Err(e) => std::panic::panic_any(e),
    };

    match nntp_stream.list() {
        Ok(groups) => {
            let mut counter = 0;
            let total_groups = groups.len();
            println!("Found {} groups altogether", total_groups);
            let step = total_groups / 10;
            for group in groups.iter() {
                // println!("Name: {}, High: {}, Low: {}, Status: {}", group.name, group.high, group.low, group.status);

                let _newgroup = create_newsgroup(&db_connection,
                                                 &group.name.to_string(),
                                                 &(group.low as i32),
                                                 &(group.high as i32));
                // println!("Saved newsgroup {}", newgroup.name);
                if (counter % step) == 0 {
                    println!("{}0%", counter / step);
                }
                counter += 1;
            }
        }
        Err(e) => std::panic::panic_any(e)
    };

    let _ = nntp_stream.quit();
}