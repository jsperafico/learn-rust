use std::fs::File;
use std::io::ErrorKind;

fn main() {
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
