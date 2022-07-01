#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message: {:#?}", &self)
    }
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let mut home = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };
    home.kind = IpAddrKind::V6(String::from("::2"));
    home.address = String::from("::2");
    println!("home: {:#?}", home);

    let home = IpAddrKind::V4(127, 0, 0, 1);
    println!("home1: {:#?}", home);

    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("loopback1: {:#?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = Option::Some(123);

    let x: i8 = 5;
    let y: Option<i8> = Option::Some(5);

    let sum = x + y; // error
}
