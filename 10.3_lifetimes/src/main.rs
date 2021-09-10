use std::fmt::Display;

//Concept of Borrow Checker will verify if all borrows are valid.
// fn dangling() {
//     //Variable declared, but can't be used before have a value assinged.
//     //There is no null in Rust, so if used, will raise compiler error.
//     let r; 
//     { 
//         let x = 5;
//         r = &x;
//     }
//     //x by going out of scope, extinguish its lifetime.
//     println!("r: {}", r);
// }

//Generic lifetime annotations parameters: it will not change the behavior of lifetime, 
//will describe the replationship of the lifetimes of multiple references to each other.
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    //No generic lifetime annotation is requred, inherited by self.
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self.part)
    }
}

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s: &'static str = "This will live for the entire duration of the program.";

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishamel. Some yers ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    println!("{}", s);

    longest_with_an_announcement(string1.as_str(), string2, i);
}

//Will not work beucase when result is last used, the reference of string2 could be invalid.
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }