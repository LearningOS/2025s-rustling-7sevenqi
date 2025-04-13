// enums1.rs

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("Hello world".to_string()));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(255, 0, 128));
}
