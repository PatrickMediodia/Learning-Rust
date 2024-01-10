fn main() {
    /* 
    Ownership Rules
    1. Each value in Rust has a variable that's called its owner
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    { // s is not valid here, it's not yet declared

        // stored on the stack
        let string_literal = "hello"; // s is valid from this point forward

        // stored on the heap
        // rust automatically allocates memory in the heap for our string
        let string_type = String::from("hello"); // dynamic in nature

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // stringType which is stored on the heap is deallocated and cannot be accessed from this point forward


    let x = 5;
    let y = x; //copy
    // 

    let s1 = String::from("hello"); // create a string type on the heap
    // let s2 = s1; // Move (not shallow copy)
    // moved not copied
    // s2 points to the starting address of the array in memory, being in new owner taking it from s1
    
    let s2 = s1.clone();
    // if you need to clone, use the .clone() method

    println!("{}, world!", s1);
    // rust defaults to moving the value on the heap (less expensive) rather than copying it
    // rust defaults to copying the value on the stack

    /* 
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);
    */
    // passing parameters from the heap into a function is the same as assining s to another variable

    let x = 5;
    makes_copy(x);
    println!("{}", x);
    // remember that integers are copied rather than moved

    let s1 = gives_ownership();
    // the string created inside the function is now giving its ownership to s1
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // create a string
    let some_string = String::from("hello");

    //return the created string
    some_string
}

// moving the value of s2 to takes_and_gives_back, and takes_and_gives_back gives it to s3
// moving ownership into a function is tedious
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

