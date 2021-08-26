fn main() {
    // Stack: string literal (&str)
    //   because we know the size from the compiler time
    //   same goes for the data types seen previously
    // Heap: object (String)
    //   we don't know the size from the compiler time

    let mut greetings = String::from("Hello");
    if greetings.len() > 1 {
        let greetings = String::from("not today");
        println!("{}", greetings);
    } //shadow variable as dropped by calling drop automatically
    // Shadowing in a different scope, don't delete refference of previous value.
    println!("{}", greetings);

    if greetings.contains("Hello") {
        greetings = String::from("maybe today");
        println!("{}", greetings);
    }
    // Replacing the content in a scope, will change the refference of the previous value.
    println!("{}", greetings);

    // The String object was moved (ptr, size and capacity), but the actual content wasn't.
    let _not_cloned = greetings;
    println!("{}", _not_cloned);
    //println!("{}", greetings); // greetings was invalidated from now on.

    // The content of x is copied to y, because integers have a defined size and are stored
    // in the stack. Meaning, copying the value is cheap and values are stored relatively
    // close to each other.
    let x = 5;
    let y = x;
    println!("({}, {})", x , y);

    take_ownership(_not_cloned);
    // println!("Not cloned {}", _not_cloned); //Variable was invalidated

    make_copy(x);
    println!("({}, {})", x , y);
}

fn take_ownership(value : String) {
    println!("Owner: {}", value);
}

fn make_copy(i : i32) {
    println!("Copy {}", i);
}