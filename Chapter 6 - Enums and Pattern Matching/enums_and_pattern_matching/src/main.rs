/*

Enums and Pattern Matching

In this chapter, we’ll look at enumerations, also referred to as enums.
Enums allow you to define a type by enumerating its possible variants.
First we’ll define and use an enum to show how an enum can encode meaning along with data.
Next, we’ll explore a particularly useful enum, called Option, which expresses that
a value can be either something or nothing. Then we’ll look at how pattern matching
in the match expression makes it easy to run different code for different values of an enum.
Finally, we’ll cover how the if let construct is another convenient and concise idiom available
to handle enums in your code.

*/

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrV2 {
    V4(String),
    V6(String),
}

enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // V2
    let home = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrV2::V6(String::from("::1"));

    // V3
    let home = IpAddrV3::V4(127, 0, 0, 1);
    let loopback = IpAddrV3::V6(String::from("::1"));

}

fn route(ip_kind: IpAddrKind) {}