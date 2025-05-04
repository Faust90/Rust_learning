//Each of "mod" is a module
// we can group definitions
mod front_of_house {

    fn entrance() {}

    // Public (explicited by "pub")
    pub mod hosting {
        // We also have to mark each thing inside a module to be Public 
        pub fn add_to_waitlist() {
            //"super::" is used to refernce the parent scope or module 
            super::entrance();
        }

        pub fn seat_at_table() {}
    }

    // Private (by default)
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

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