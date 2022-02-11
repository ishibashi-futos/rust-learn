pub fn defining_enum() {
    let home = IpAddr::V4(127, 0, 0, 1);
    home.print();
    let loopback = IpAddr::V6(String::from("::1"));
    loopback.print();

    let some_number = Some(5);
    println!("{:?}", some_number);
    let some_string = Some("a string");
    println!("{:?}", some_string);
    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);

    let quit = Message::Quit;
    quit.call();
    let mv = Message::Move(MoveMessage {x: 10, y: 10});
    mv.call();
    let write = Message::Write(String::from("hello"));
    write.call();
    let change_color = Message::ChangeColor(255, 0, 255);
    change_color.call();
}


enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => println!("{}", addr),
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move(MoveMessage),
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}