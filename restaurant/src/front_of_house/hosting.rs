//NOTE: the filename must be the same as the module name to be valid

// Public (explicited by "pub")
use crate::front_of_house;

// We also have to mark each thing inside a module to be Public
pub fn add_to_waitlist() {
    front_of_house::entrance();
}

pub fn seat_at_table() {}
