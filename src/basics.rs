// ############# Variables

pub fn print_variables() {
    // `mut`` allow us to change the value of the variable.
    let mut x: i32 = 4;
    x += 1;
    
    println!("This is x: {}", x);
    
    // Unlike other languages, we are able to declare a variable with the same name again.
    // CONCEPT: SHADOWING
    let _shadowing: i32 = 5;
    let _shadowing = 6;
    
    let _spaces = "    ";
    // By shadowing, we can change the values of a data like this.
    let _spaces = _spaces.len();
    
    let mut _spaces = "    ";
    // spaces = spaces.len(); This will not work because spaces in the above line sets type to String
    

    // The type and value of a CONST cannot change throughout the program.
    // Similar to Java's static final, or C's const
    const SPEED_OF_LIGHT: u32 = 299792458;
    println!("The speed of light in m/s is: {}", SPEED_OF_LIGHT);
}

// ############# Data Types

// Scalar Types = Represent single value
// 1. Integer Types (i8, i16, i32, i64, i128, isize)
// 2. Floating Types (f32, f64)


