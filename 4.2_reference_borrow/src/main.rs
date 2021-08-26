fn main() {
    let mut value = String::from("should do this");
    reference(&value);

    value.push_str(", not I won't");
    println!("{}", value);

    mutable_reference();
}

fn reference(value : &String) {
    println!("Using the refference of '{}'", value);
}

fn mutable_reference() {
    let mut greeting = String::from("Hello");

    let h3 = &greeting;
    let h4 = &greeting;
    println!("H3 '{}', H4 '{}'", h3, h4);

    let h1 = &mut greeting;
    println!("H1 '{}'", h1);

    // Unable to use immutable reference after mutable reference declaration.
    //println!("H3 '{}', H4 '{}'", h3, h4); 

    // Is not possible to have more than 1 mutable reference within the same scope.
    // let h2 = &mut greeting; 
    //println!("H1 '{}', H2 '{}'", h1, h2);
}

// Can't return a reference of a dropped object.
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}