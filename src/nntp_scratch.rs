use nntp::{Article, NNTPStream};

fn nntp_main() {
    let mut nntp_stream = match NNTPStream::connect(("nntp.aioe.org", 119)) {
        Ok(stream) => stream,
        Err(e) => std::panic::panic_any(e),
    };

    match nntp_stream.capabilities() {
        Ok(lines) => {
            for line in lines.iter() {
                print!("{}", line);
            }
        }
        Err(e) => std::panic::panic_any(e),
    }

    // match nntp_stream.list() {
    // 	Ok(groups) => {
    // 	    for group in groups.iter() {
    // 		println!("Name: {}, High: {}, Low: {}, Status: {}", group.name, group.high, group.low, group.status)
    // 	    }
    // 	},
    // 	Err(e) => std::panic::panic_any(e)
    // };

    // match nntp_stream.group("comp.sys.raspberry-pi") {
    //     Ok(_) => (),
    //     Err(e) => std::panic::panic_any(e),
    // }

    // match nntp_stream.article_by_number(6187) {
    //     Ok(Article { headers, body }) => {
    //         for (key, value) in headers.iter() {
    //             println!("{}: {}", key, value)
    //         }
    //         for line in body.iter() {
    //             print!("{}", line)
    //         }
    //     }
    //     Err(e) => {
    //         println!("Can't get article by number");
    //         // std::panic::panic_any(e)
    //     }
    // }
    //
    // match nntp_stream.article_by_id("<cakj55F1dofU5@mid.individual.net>") {
    //     Ok(Article { headers, body }) => {
    //         for (key, value) in headers.iter() {
    //             println!("{}: {}", key, value)
    //         }
    //         for line in body.iter() {
    //             print!("{}", line)
    //         }
    //     }
    //     Err(e) => {
    //         println!("Couldn't get article by ID");
    //         // std::panic::panic_any(e)
    //     }
    // }

    let _ = nntp_stream.quit();
}