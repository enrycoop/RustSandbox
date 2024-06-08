/*
The Rust community has developed guidelines for splitting the separate concerns
of a binary program when main starts getting large. This process has the
following steps:
  1) Split your program into a main.rs and a lib.rs and move your program’s logic
    to lib.rs.
  2) As long as your command line parsing logic is small, it can remain in main.rs.
  3) When the command line parsing logic starts getting complicated, extract it
    from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should
be limited to the following:
  1) Calling the command line parsing logic with the argument values
  2) Setting up any other configuration
  3) Calling a run function in lib.rs
  4) Handling the error if run returns an error
*/

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
