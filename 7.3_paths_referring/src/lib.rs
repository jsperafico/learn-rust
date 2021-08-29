// Don't need pub, because is being defined withtin the same module is using it.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn apologies() {}
    }
    pub mod serving {
        pub fn take_order() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        super::front_of_house::hosting::apologies()
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
    self::front_of_house::serving::take_order();

    self::back_of_house::fix_incorrect_order();
}