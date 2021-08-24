use std::{
    io, //Like c++
    cmp::Ordering
};
use rand::Rng;

fn main() {
    println!("Hey, how about guessing a number?");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess...");
        let mut guess = String::new(); //By default variables aren't mutables

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read your input.");

        let guess : u32 = match guess.trim().parse() {
            //shadowing previous value of guess with a new value
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue
            }
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            // The following arms for the given matching
            // Each arm consist on a pattern
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Perfect");
                break;
            }
        };
    }
}
