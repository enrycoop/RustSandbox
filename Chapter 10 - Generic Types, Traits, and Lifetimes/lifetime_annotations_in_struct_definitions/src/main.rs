// L'istanza di Important Exception non può vivere di più del riferimento
// a part. Excerpt = estratto
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Elision è quando il compilatore inferisce la lifetime
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}




fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
    // first_sentence è una string slice
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);

    println!("{}", i.level());
    let part = i.announce_and_return_part("yeahhh!!");
    println!("returns {part}");

    // dura per tutto il programma
    let s: &'static str = "I have a static lifetime.";
    let part = longest_with_an_announcement(novel.as_str(), s, "Announce!!");
    println!("ret {part}");
}

use std::fmt::Display;
// All together
fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}