fn main() {
    let ttc = format!("{}-{}-{}", "tic", "tac", "toe");
    println!("Result: {}", ttc);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let result = tic + &tac + &toe;

    println!("Result: {}", result);

    // Tic will be invalid beucase of add(self, s &str) -> String
    //println!("Tic: {}", tic);

    // Index can't be accessed in String, nor &str
    //let value = &tac[1];
    let value = &result.chars().nth(1);
    println!("Chars[1] {}", value.expect("Not found"));
}
