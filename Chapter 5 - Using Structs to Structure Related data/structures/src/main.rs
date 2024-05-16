/*

Using Structs to Structure Related Data

A struct, or structure, is a custom data type that lets you package together and name
multiple related values that make up a meaningful group.
If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.
In this chapter, we’ll compare and contrast tuples with structs to build on what you already know
and demonstrate when structs are a better way to group data.

We’ll demonstrate how to define and instantiate structs. We’ll discuss how to define associated
functions, especially the kind of associated functions called methods, to specify behavior associated 
with a struct type. Structs and enums (discussed in Chapter 6) are the building blocks for creating 
new types in your program’s domain to take full advantage of Rust’s compile-time type checking.

*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Mario Rossi"),
        email: String::from("mario.rossi@mail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("m.rossi@mail.com");

    // Struct Update Syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // Note that the struct update syntax uses = like an assignment; this is because it moves the data
    let user2 = User {
        email: String::from("m.rossi@mail.com"),
        ..user1
    };
    println!("{}", user2.email);

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("({}, {}, {})", black.0, black.1, black.2);
    println!("({}, {}, {})", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

// Tuple Structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
