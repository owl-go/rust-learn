use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parse arguments:{}", err);
        process::exit(1);
    });
    println!("query:{},filename:{}", config.query, config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("application err:{}", e);
        process::exit(1);
    }
}
