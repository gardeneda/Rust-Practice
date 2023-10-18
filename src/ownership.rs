#![allow(dead_code)]

// TRAIT: COPY, DROP

// BACKGROUND INFORMATION - OWNERSHIP, UNIQUE TO RUST.

// Unlike C where we explicitly allocate and free memory,
// Rust manages memory through a system of ownership with a set of
// rules that the compiler checks.
// None of the features will slow down the program when it is running.

// Re-Emphasis: Accessing data in the heap is slower than 
// accessing data on the stack because you have to follow a pointer to get there.

// Contemporary processors are faster if they jump around less in memory. 
// Continuing the analogy, consider a server at a restaurant taking orders from many tables. 
// It’s most efficient to get all the orders at one table before moving on to the next table. 
// aking an order from table A, then an order from table B, then one from A again, and then one
// from B again would be a much slower process. By the same token, a processor can do its job better if
// it works on data that’s close to other data (as it is on the stack) 
// rather than farther away (as it can be on the heap).

// When your code calls a function, the values passed into the function (including, potentially, 
// pointers to data on the heap) and the function’s local variables get pushed onto the stack. 
// When the function is over, those values get popped off the stack.

// CONCLUSION: Keeping track of what parts of code are using what data on the heap, 
// minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap 
// so you don’t run out of space are all problems that ownership addresses.

// ##############################
// OWNERSHIP RULES
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. If owner goes out of scope, value will be dropped.


pub fn ownership() {
    // Immutable String Type. Known at compile time, since it's hardcoded.
    // this scope is now over, and s is no longer valid
    let _s: &str = "String Literal";
    
    // Rust has a second string type, String. 
    // This manages data allocated on the heap and stores an amount of text
    // that is unknown to us at compile time.
    let mut _s: String = String::from("hello");
    // This is mutable. To support a mutable, growable piece of text,
    // we need to allocate amount of memory on the heap, unknown at compile time.
    _s.push_str(", world!");
    println!("{}", _s);

    {
        let _disappears_after_socpe: String = String::from("Test Literal");

    } // the variable above is no longer in the scope, and is freed from memory
    // To achieve freeing of memory, it calls a function called `drop`.
    // Also known as RAII (Resource Acuiisition Is Initialization)
    let s1: String = String::from("hello");
    let _s2: String = s1;
    // println!("{} This returns an error!", s1);
        // Will return an error, because s1 becomes an invalidated reference.
        // This is to prevent the DOUBLE FREE ERROR.
        // It's considered a MOVE. 

    // If we want to deeply copy the HEAP data of the String
    // Not just the stack data;
    let s1: String = String::from("hello");
    let s2: String = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // The same behavior above isn't seen in this example:
    // This is shallow copying.
    // Integer is a known size at compile time and stored on stack.
    // That means there’s no reason we would want to prevent x from being valid 
    // after we create the variable y.

    let x: i32 = 5;
    let y: i32 = x;

    println!("x = {}, y = {}", x, y);

    // Passing a value to a function is similar 
    // to assigning a value to a variable; it will MOVE or get COPIED

    let s = String::from("hello");  // s comes into scope

    _takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    _makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let _s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.



}

fn _takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn _makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

a_string  // a_string is returned and moves out to the calling function
}