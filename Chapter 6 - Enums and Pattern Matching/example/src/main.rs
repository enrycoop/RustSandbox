
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
           Message::Write(msg) => println!("{}", msg),
           Message::Move { x, y } => println!("{{{}, {}}}", x, y),
           Message::ChangeColor(r, g, b) => println!("({r}, {g}, {b})"),
           _ => (),
        }
    }
}


fn main() {
    let messages = [
        Message::Write(String::from("hello")),
        Message::Move { x: 5, y: 6 },
        Message::ChangeColor(011, 12, 14),
        Message::Quit,
    ];
    for m in messages {
        m.call();
    }

}
