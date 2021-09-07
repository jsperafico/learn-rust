pub mod guess;

use std::{
    io,
    net::IpAddr,
    cmp::Ordering
};
use rand::Rng;
use guess::Guess;

fn main() {
    let value = String::from("127.0.0.1");
    print_home(&value);
    guessing();
}

// Building api, if parameters don't make sense, just panic.
fn print_home(value : &String) {
    let home : IpAddr = value.parse().unwrap();
    println!("Home: {:?}", home);
}

fn guessing() {
    println!("Hey, how about guessing a number?");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess...");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read your input.");

        let guess : Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Please input a number!");
                continue
            }
        };

        println!("You guessed: {}", guess.value());
        
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Perfect");
                break;
            }
        };
    }
}
