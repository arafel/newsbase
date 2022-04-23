use clap::{Arg, Command, ArgGroup};

fn build_parser<'help>() -> Command<'help> {
    let m = Command::new("newsbase")
        .author("Paul Walker <paul@blacksun.org.uk>")
        .version("0.1.0")
        .about("Does stuff")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("groups")
                .arg(Arg::new("update")
                    .short('u')
                    .long("update"))
                .arg(Arg::new("search")
                    .alias("find")
                    .short('s')
                    .long("search")
                    .takes_value(true))
                .group(ArgGroup::new("group_commands")
                    .args(&["update", "search"])
                    .required(true))
        )
        .subcommand(
            Command::new("articles")
                .arg(Arg::new("search")
                    .alias("find")
                    .short('s')
                    .long("search")
                    .takes_value(true))
                .group(ArgGroup::new("article_commands")
                    .args(&["search"])
                    .required(true))
        );
    m
}

fn main() {
    let parser = build_parser();
    let args = parser.get_matches();

    match args.subcommand() {
        Some(("groups", group_matches)) => {
            if group_matches.is_present("search") {
                println!("search is present, finding things to search for");
                let group = group_matches.value_of("search").unwrap();
                println!("Searching for {}...", group);
                return;
            }

            if group_matches.is_present("update") {
                println!("Updating groups");
            }
        }
        Some(("articles", article_matches)) => {
            if article_matches.is_present("search") {
                println!("search is present, finding things to search for");
                let group = article_matches.value_of("search").unwrap();
                println!("Searching for {}...", group);
                return;
            }
        }
        _ => {
            // FIXME - not the most elegant way to handle this!
            unreachable!()
        }
    }
}
