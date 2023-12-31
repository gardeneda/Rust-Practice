// In Rust, the main.rs is a crate, and there is no implicit mapping between file systems.
// We need to build it.

// Example of importing specific functions: < use crate::basics::print_variables; >

mod basics;
mod ownership;

fn main() {
    ("Hello, world!");
    basics::print_variables();
    basics::print_scalar_data_types();
    basics::print_compound_data_types();
    basics::print_labeled_measurement(15, "kg");
    basics::this_scary_loop();
    basics::disambiguate_loops();
    basics::for_loop();
    ownership::ownership();
}
