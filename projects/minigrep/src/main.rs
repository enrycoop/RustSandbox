use std::{env, process};
use minigrep::Config;

fn main() {
    // Reading arg values
    let args: Vec<String> = env::args().collect();

    // Handling errors on arg values
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
