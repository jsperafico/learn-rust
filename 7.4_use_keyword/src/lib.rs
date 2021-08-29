mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    pub mod serving {
        pub fn take_order() {}
    }
}


// use crate goes to the root of this crate.
//use crate::front_of_house::hosting;

// use self idetify within the same file.
use self::front_of_house::hosting;

use self::front_of_house::serving as s;

//glob operator
//use self:;front_of_house::*;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    s::take_order();
}