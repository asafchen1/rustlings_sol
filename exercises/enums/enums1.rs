// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit, Echo, Move, ChangeColor,
}

fn main() {
    let a = Message::Quit;
    println!("{:?}", a);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
