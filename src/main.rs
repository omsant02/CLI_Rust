use std::env; // to read the values of command line arguments
// use std::fs;  // to handle files
use std::process;
// use std::error::Error;

use minigrep::Config;

fn main() {

    // passing ownership of the iterator returned from env::args to Config::build directly.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

