fn main() {
    println!("Hello, world!");
    another_function();

    println!("Sum {}", sum(1,2));
    println!("Complexity {}", complexity(10));
}

fn another_function() {
    // Statement: series of instructions to perform action without return value.
    let _x = 0;
    // let _x = (let _y = 0); // let is a statement, don't have a return value.

    println!("Another function!");
}

// Expression: evaluate and return something.
fn sum(a : i32, b : i32) -> i32 {
    a + b //tail expression
}

fn complexity(a: i32) -> i32 {
    // Also an expression, beucase of the brackets.
    let z = {
        let y = a * 2;
        y + 10 //tail expression
    };
    z - 5 //tail expression
}