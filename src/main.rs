// In Rust, the main.rs is a crate, and there is no implicit mapping between file systems.
// We need to build it.

// Example of importing specific functions: < use crate::basics::print_variables; >

mod basics;

fn main() {
    ("Hello, world!");
    basics::print_variables();
}