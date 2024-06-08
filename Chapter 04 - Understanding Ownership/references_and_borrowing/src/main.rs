/*************************************************************/
// The Rules of References
//
// Let’s recap what we’ve discussed about references:
//
//  -  At any given time, you can have either one mutable reference or any number of immutable references.
//  -  References must always be valid.
//
/************************************************************/


fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    change(&mut s); // Borrowing
    //Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
    println!("{}", s);

    // DANGLING REFERENCES
    let _reference_to_nothing = no_dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s // ownership is moved out, and nothing is deallocated.
}