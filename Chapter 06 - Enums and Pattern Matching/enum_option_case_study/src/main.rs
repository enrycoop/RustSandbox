fn main() {
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    let x: Option<&str> = None;
    x.expect("fruits are healthy"); // panics with `fruits are healthy`

}
