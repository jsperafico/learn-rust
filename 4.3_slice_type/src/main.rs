fn main() {
    let mut phrase = String::from("Hey! You!");
    let index = first_word(&phrase);

    // Will make the program panic by trying to access an invalid index
    //phrase = String::from("a");
    phrase = phrase.replace("Hey!", "Hello");

    //Slices don't have ownership. Only reference is allowed.
    let sliced : &str = &phrase[0..index];

    //Conflict of mutable and immutable reference usage.
    //phrase.clear();

    println!("Print the following '{}", sliced);
    println!("Simplified slice '{}'", new_first_word(&phrase));

    println!("From beginning '{}'", &phrase[..5]);
    println!("Until the end '{}'", &phrase[5..]);

    //String literal are slices by default and immutable as well
    let literal : &str = "Hey sir";
    println!("Print liretal '{}", literal);
    println!("Simplified slice '{}'", new_first_word(literal));
}

fn first_word(value : &String) -> usize {
    let bytes = value.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    value.len()
}

fn new_first_word(value : &str) -> &str {
    let bytes = value.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &value[0..i];
        }
    }
    // '..' = full range of values within the array
    &value[..]
}