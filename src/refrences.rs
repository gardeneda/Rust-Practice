#![allow(dead_code)]
// Basic background knowledge requires you to understand Ownership
// before proceeding with this page.


// Very similar to passing pointers in C
pub fn borrow() {
    let s1 = String::from("hello");

    // This is not modifiable.
    // Refer to comments on the function, located beneath this function.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("Mutable String");

    // We can create a borrow out on the side.
    let r1 = &mut s2;

    /* 
        However, we cannot have multiple mutable references to the same value
        This is to prevent multiple mutable references to the same data at the same time.
        This allows for mutation but in a very controlled fashion.
    */
    
    // let r2 = &mut s2;

    /* 
        A data race is similar to a race condition and happens when these three behaviors occur:
        Two or more pointers access the same data at the same time.
        At least one of the pointers is being used to write to the data.
        There’s no mechanism being used to synchronize access to the data.
        Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; 
        Rust prevents this problem by refusing to compile code with data races!
    */

    // Because the value is not mutable, we can now change the value from here.
    let mut mut_len = mutable_calculate_length(&mut_len);

    // Prevent dangling references.
    // let _reference_to_nothing = dangle();

}

// Don't need to return values because we never owned it in the first place.
// Creating a reference is called (borrowing)
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.


// By making the type a mutable address 
fn mutable_calculate_length(s: &mut String) -> i32 {
    s.push_str("append this");
    s.len()
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!