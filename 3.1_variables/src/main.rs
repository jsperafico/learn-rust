const CONSTANT_VALUE : &str = "should not be changed";

fn main() {
    //let x = 6; // By default is immutable
    let mut x = 6; // 'mut' will make it mutable
    println!("This is the value of x = {}", x);
    x = 5;
    //shadowing the previous value
    // changing the type and value, being mutable or immutable is irrelevant
    let x = x.to_string();
    println!("This is the new value of x = {}", x);

    println!("Example of constant value '{}'", CONSTANT_VALUE);
}
