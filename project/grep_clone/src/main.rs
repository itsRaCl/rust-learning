use std::env;
use std::process;

use grep_clone::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|error_string| {
        eprintln!("{error_string}");
        process::exit(1);
    });

    if let Err(e)= grep_clone::run(config){
        eprintln!("Application Error {e}");
        process::exit(1);
    }
}

