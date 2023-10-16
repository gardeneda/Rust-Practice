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
// 1. Integer Types  DEFAULT: i32
//        Signed: (i8, i16, i32, i64, i128, architecture: isize)
//      -2^7 ~ 2^7 -1 :: Two's Complement

//      Unsigned: (u8, u16, u32, u64, u128, architecture: usze)
//      0 ~ 2^8 -1

//      Integer Overflow is possible! (which will trigger a `panic!` in Rust)

// 2. Floating Types (f32, f64)
//          f32: 32-bit floating (single-precision)
// DEFAULT  f64: 64-bit floating 

// 3. Boolean (bool)

// 4. Char (any single char) - **IS 4 BYTES in RUST 

pub fn print_scalar_data_types() {
    
    let y: u32 = 178;
    // let x: u32 = -178; // Will cause error;
    
    let floating_point = 27.2;

    let this_is_true: bool = true;
    let this_is_false: bool = false;
    
    let char_x: char = 'x';

    println!("{}, {}, {}, {}, {}", y, floating_point, this_is_true, this_is_false, char_x);
}


pub fn print_compound_data_types() {

}


