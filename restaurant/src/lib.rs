mod front_of_house; //taken from front_of_house. IS NOT LIKE A IMPORT!

// using the scope in a way such that hosting can be used on is own 
use crate::front_of_house::*;

pub fn eat_at_restaurant() {
    //Absolute path
    hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::seat_at_table();
}

mod back_of_house {

    // Evenc structs must adhere to the principle of "private default" also for each field inside of them
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}