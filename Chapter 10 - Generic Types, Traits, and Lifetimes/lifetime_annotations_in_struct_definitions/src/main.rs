// L'istanza di Important Exception non può vivere di più del riferimento
// a part.
struct ImportantException<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
    // first_sentence è una string slice
    let i = ImportantException {
        part: first_sentence,
    };

    println!("{}", i.part);

}
