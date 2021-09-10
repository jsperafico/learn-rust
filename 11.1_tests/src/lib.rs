#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value <= 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value >= 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 6,
            height: 2,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test] //Atribute with metadata
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 6,
            height: 2,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        // assert_ne!(5, 5); // panicked at 'assertion failed: `(left != right)`
    }

    #[test]
    #[should_panic]
    fn should_not_say_hey() {
        assert!(
            greeting("Sandra").contains("Hey"),
            "Greeting did not contain the expression '{}'",
            "Hey"
        )
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn guess_grater_than_100() {
        Guess::new(101);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus tow does not equal four"))
        }
    }
}
