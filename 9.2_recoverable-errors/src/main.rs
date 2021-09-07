use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

// Unable to have automatic throw on a main function.
//fn main() -> Result<(), Box<dyn Error>> {
//    let f = File::open("hello.txt")?;
//    Ok(())
//}

fn main() {
    println!("{:#?}", read_username_short_approach());
    println!("{:#?}", read_username_different_approach());
    println!("{:#?}", read_username_from_file());
    unwrap_expect();
    unwrapable();
    matchable();
}

fn read_username_short_approach() -> Result<String, io::Error> {
    let mut s = String::new();

    //? can only be used with return Result. It will propagate the Error.
    File::open("hey.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_different_approach() -> Result<String, io::Error> {
    //? can only be used with return Result. It will propagate the Error.
    let mut f = File::open("test.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error)
    }
}

fn unwrap_expect() {
    //let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Unable to load file");
}

fn unwrapable() {
    let _f = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn matchable() {
    let f = File::open("test.txt");

    let _f = match f {
        Ok(value) => value,
        Err(value) => match value.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(creation) => creation,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _other_error => panic!("Error: {:#?}", _other_error),
        }
    };
}
