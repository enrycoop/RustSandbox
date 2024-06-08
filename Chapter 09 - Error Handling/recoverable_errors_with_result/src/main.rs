/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
T ed E sono tipi generici: discussi nel cap 10. T è il tipo di valore che viene
restituito in caso di successo ed E in caso di errore.

 */
// File handler
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // il tipo E usato qui è std::io::Error
    let greeting_file_result = File::open("hello.txt");


    // Da notare che Option e Result enum sono portate nello scope da prelude
    // quindi non dobbiamo specificare Result:: prima di Ok e Err nel match.
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    let unexistent_file = "helloooouwwww.txt";
    let _greeting_file = File::open(unexistent_file)
        .expect(format!("{} should be included in this project", unexistent_file).as_str());

}
