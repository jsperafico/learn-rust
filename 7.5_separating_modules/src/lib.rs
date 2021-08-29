// Load as module the content of another file in the same location as this file.
mod front_of_house;

// Extract and load hosting module withtin a given path and expose it to whom will consume this lib crate.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}