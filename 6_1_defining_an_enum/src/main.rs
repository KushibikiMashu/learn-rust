enum IpAddr {
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
        // メソッド本体
        println!("{:#?}", &self);
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // 共にIpAddr型
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    // IpAddr型を引数に取る
    // fn route(id_type: IpAddr) { }

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    // Tにi32型を指定する
    let absent_number: Option<i32> = None;
}
