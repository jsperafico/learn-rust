use rand::Rng;

fn main() {
    let x = rand::thread_rng().gen_range(1..=10);
    if x < 5 {
        println!("Lower than 5");
    } else {
        println!("Greater or equal to 5");
    }
    println!("Given value {}", x);

    evaluate(x, 6);

    let y = if x % 2 == 0 { 1 } else { 4 };
    println!("Y is {}", y);

    again();
    speak();
    loop_array();
}

fn evaluate(number: i32, compare: i32) {
    if number < compare {
        println!("Lower than {}", compare);
    } else if number == compare {
        println!("Equal to {}", compare);
    } else {
        println!("Greater than {}", compare);
    }
}

fn again() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is {}", result);
}

fn speak() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("Exited!");
}

fn loop_array() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter().rev() {
        println!("The element is: {}", element);
    }
}