use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;

// longer
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//     Ok(_) => Ok(username),
//     Err(e) => Err(e),
//     }
// }

// Shorter
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// Shorter
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// Shorter
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

// fn main() {
    
//     match read_username_from_file() {
//         Ok(username) => {
//             println!("The username is {}", username);
//             match last_char_of_first_line(&username) {
//                 Some(c) => println!("Last char on first line is: {c}"),
//                 None => println!("No char into file..."),
//             }
//         },
//         Err(e) => println!("{}", e),
//     }
    
// }


// Non compila perché l'operatore ? può essere usato solo in una funzione
// che restituisce Result, Option o un altra funzione che implementa FromResidual
// fn main() {
//     let greeting_file = File::open("hello.txt")?;

// }


fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
