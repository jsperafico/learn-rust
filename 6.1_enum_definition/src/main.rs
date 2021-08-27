#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, //Anonymous struct
    Write(String), 
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => return,
            Message::Move{x,y} => println!("{},{}", x, y),
            Message::Write(value) => println!("{}", value),
            Message::ChangeColor(r,g,b) => println!("({},{},{})", r, g, b),

        };
    }
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move{x: 2, y: 5};
    let w = Message::Write(String::from("Hey"));
    let c = Message::ChangeColor(1, 0, 0);

    q.call();
    m.call();
    w.call();
    c.call();

    let name: Option<&str> = Some("Jonathan");
    let empty: Option<&str> = None;
    println!("My name is '{:?}'", name);
    println!("How about empty '{:?}'", empty);
}
