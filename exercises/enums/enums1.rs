// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM NOT DON

#[derive(Debug)]
enum Message {
    Quit(bool),
    Echo(String),
    Move(usize),
    ChangeColor(usize,usize)
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit(true));
    println!("{:?}", Message::Echo("sdss".to_string()));
    println!("{:?}", Message::Move(100));
    println!("{:?}", Message::ChangeColor(1,2));
}
