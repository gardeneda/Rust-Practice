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

// Tuple Type
// General way of grouping together a number of values with a variety of types.

// Array Type
// Arrays allocate data on the stack raher than the heap - just like C langauge.

pub fn print_compound_data_types() {
    // ### Tuple
    let tuple: (i32, f64, u8) = (5002, 12.5, 7);
    let tuple2: (i32, char, bool) = (512, 'x', true);

    // To get the value of a tuple, dereference it like you would in JS
    let (a, _b, _c) = tuple;
    let (_x, y, _z) = tuple2;
    println!(
        "Expected value: 5002, Actual value: {} | Expected value: x, Actual value: {}", 
        a, y
    );

    // We can also access a tuple by using the (.) notation.
    let five_thousand = tuple.0;
    let two_pwr_nine = tuple2.0;
    println!(
        "Expected: 5002 ? {} | Expected: 512 ? {}",
        five_thousand, two_pwr_nine
    );    

    // Tuple without any values is called a 'unit'.

    // ### Array
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Init array with same value for each element:
    let arr2 = [3; 5]; // Returns 5 element that are value 3.

    // We can access arrays like how we always do:
    println!("This is 3: {}", arr2[0]);

    // Does it print garbage like C if we refer to an element out of bounds?
    // println!("Expect error: {}", arr2[15]);

}

// As long as the function is in the scope where it can be seen by the caller,
// it doesn't mean where it is. (Contrary to JavaScript - excluding Hoisting) || Also similar to Java in such regard.
fn _foo() {
    println!("This is a function!");
}

// We must define the function signature - each type for each argument.
fn _fn_parameter(x: i32) {
    println!("This is the given value: {x}");
}

// ***Rust is an expression-based language.
// Statements are instructions that perform some action and do not return values.
// Expressions evaluate to a resultant value.

// The return value is specified with an arrow: -> 
fn _five() -> i32 {
    // No semicolon or anything, because this is an expression.
    5
}

pub fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {value}{unit_label}");
}

// All functions that are expressions will be the `return`.
// Unlike many languages where we specify the `return` keyword, 
// this is different in Rust.

