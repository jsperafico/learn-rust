#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float{currency: f64},
    Text(String),
}

fn main() {
    super_v();
    super_fibonacci();
    super_flexible();
}

fn super_flexible() {
    let spreadsheet : Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float{currency: 1.1},
        SpreadsheetCell::Text(String::from("Hello")),
    ];
    println!("Spreadsheet: {:#?}", spreadsheet);

    for cell in &spreadsheet {
        match cell {
            SpreadsheetCell::Int(value) => println!("Value: {}", value),
            SpreadsheetCell::Float{currency: value} => println!("Value: {}", value),
            SpreadsheetCell::Text(value) => println!("Value: {}", value),
        }
    }
}

fn super_v() {
    let v : Vec<bool> = vec!();
    println!("v's value: {:?}", v);

    // Shadowing previous declaration and reference for v.
    let mut v = vec![1,2,3,4,5];
    println!("New v reference: {:?}", v);

    // Immutable reference
    for i in &v {
        println!("{}", i);
    }

    // Mutable reference
    for i in &mut v {
        *i += 50;
    }

    println!("v's value: {:?}", v);
} 

fn super_fibonacci() {
    let mut fibonacci : Vec<i32> = Vec::new();
    fibonacci.push(0);
    fibonacci.push(1);
    fibonacci.push(1);
    fibonacci.push(2);
    fibonacci.push(3);
    fibonacci.push(5);
    fibonacci.push(8);
    fibonacci.push(13);
    println!("Fibonacci's value: {:?}", fibonacci);

    // Can't borrow as immutable and also as mutable aftwards within the same scope.
    // Nor have 2 mutable references.
    //let seventh : &i32 = &fibonacci[6];
    //fibonacci.push(21);

    let seventh : i32 = fibonacci[6];
    fibonacci.push(21);
    println!("Seventh value of fibonacci sequence: {}", seventh);
    println!("Fibonacci's value: {:?}", fibonacci);

    let nineth : &i32 = &fibonacci[8];
    match fibonacci.get(10) {
        Some(value) => println!("Immutable reference an element: {}", value),
        None => println!("Nothing")
    }
    println!("Nineth fibonacci value: {}", nineth);

    fibonacci.push(34);
    
    // Retrieve mutable refference of an vec element.
    let h : &mut i32 = &mut fibonacci[1];
    // Dereference the mutable reference to change it's value
    *h = 10;
    println!("Not a Fibonacci's anymore: {:?}", fibonacci);
}
