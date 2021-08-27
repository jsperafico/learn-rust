//Access in tuples requred a specif order, access on structs no.

// Entity can't be partially mutable
struct User {
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool,
}

// tuple structs - a way to name tuples to diferentiate from the rest.
struct Color(u8, u8, u8);

// unit-like structs
//struct Exists();

fn main() {
    let pedro = User {
        email: String::from("pedro@company.com"),
        username: String::from("not a valid username"),
        sign_in_account: 0,
        active: false
    };

    // Impossible to assign if variable is declared as immutable
    //pedro.email = pedro.email.replace("company", "anything");

    print_user(&pedro);

    let mut anna = User {
        email: String::from("anna@company.com"),
        username: String::from("maybe a valid username"),
        sign_in_account: 1,
        active: true
    };
    anna.username = String::from("Information was replaced");
    print_user(&anna);

    let catarina = build_user(
        String::from("catarina@company.com"),
        String::from("catarina")
    );
    print_user(&catarina);

    let catarina_updated = User {
        sign_in_account: 10,
        active: false,
        ..catarina
    };

    // Original Object was invalidated, due change of ownership of String values
    //print_user(&catarina);
    print_user(&catarina_updated);

    let blue = Color(0, 0, 255);
    println!("R '{}'", blue.0);
    println!("G '{}'", blue.1);
    println!("B '{}'", blue.2);
}

fn build_user(email : String, username : String) -> User {
    User {
        email, // field init shorthand syntax
        username, // field init shorthand syntax
        sign_in_account: 1,
        active: true
    }
}

fn print_user(user : &User) {
    println!("==================");
    println!("Please print email '{}'", user.email);
    println!("Please print username '{}'", user.username);
    println!("Please print account '{}'", user.sign_in_account);
    println!("Please print active '{}'", user.active);
    println!("==================");
}
