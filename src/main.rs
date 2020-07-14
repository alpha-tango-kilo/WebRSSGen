use std::env;
use std::process;

use web_rss_gen::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    web_rss_gen::run(config);
}
