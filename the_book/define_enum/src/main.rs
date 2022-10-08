
fn main() {

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let quit_message = Message::Quit;

    let move_message = Message::Move {x: 3, y: 1};

    let write_message = Message::Write(String::from("hello world"));

    let change_color_message = Message::ChangeColor(3, 1, 2);
}

enum IpAddr {
    V4(String),
    V6(String)
}


enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}