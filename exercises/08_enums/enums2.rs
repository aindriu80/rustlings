#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct ResizeMessage {
    width: u64,
    height: u64,
}

#[derive(Debug)]
struct MoveMessage {
    point: Point,
}

#[derive(Debug)]
struct EchoMessage {
    date: String,
}

#[derive(Debug)]
struct ChangeColorMessage {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize(ResizeMessage),
    Move(MoveMessage),
    Echo(EchoMessage),
    ChangeColor(ChangeColorMessage),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize(ResizeMessage {
            width: 10,
            height: 30,
        }),
        Message::Move(MoveMessage {
            point: Point { x: 10, y: 15 },
        }),
        Message::Echo(EchoMessage {
            date: String::from("hello world"),
        }),
        Message::ChangeColor(ChangeColorMessage {
            r: 200,
            g: 255,
            b: 255,
        }),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
