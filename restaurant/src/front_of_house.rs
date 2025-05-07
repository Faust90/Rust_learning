//NOTE: the filename must be the same as the module name to be valid

fn entrance() {}

pub mod hosting;

// Private (by default)
mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
