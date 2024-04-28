/*
The Slice Type

Slices let you reference a contiguous sequence of elements in a collection
 rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.
*/

fn main() {
    let mut s = String::from("Hello world!");
    let word = first_word(&s);
    println!("First word ends {word}");
    s.clear(); // this empties the String, making it equal to ""
    // println!("String cleaned: {word}"); error!! the s is empty

    let s = String::from("Hello world");

    //[starting_index..ending_index] ending_index is one more than the last position in the slice.
    let hello = &s[0..5];
    let world = &s[6..];

    println!("{hello} and {world}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("1 {word}");
    let word = first_word(&my_string_literal[..]);
    println!("2 {word}");
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("3 {word}");

    // other slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { //
            return &s[0..i];
        }
    }

    &s[..]
}
