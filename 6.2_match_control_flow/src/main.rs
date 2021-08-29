#[derive(Debug)]
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("How luck are you, right?");
            1
        },
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alabama);
    println!("Penny: {}", value_in_cents(penny));
    println!("Quarter: {}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}", six);
    println!("None: {:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("tree"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("other"),
    }
}
