/* 
fn main() {
    //let s1 = String::from("hello");
    //let len = calculate_length(&s1);
    //println!("The length of '{}' is {}.", s1, len);

    // borrowing/references are immutable by default
    // but if you wanted to, make sure the variable is mutable and pass a mutable reference
    let mut s1 = String::from("hello");
    change(&mut s1);
}
*/

/*
//let (s2, len) = calculate_length(s1);
// string as parameters and get a tuple of (string, length) back
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
*/

/* 
let len = calculate_length(s1);
//instead of taking a string, you could borrow it by taking its address
fn calculate_length(s: String) -> usize {
    let length = s.len();

    length
}
*/

//s is now a reference to the string and does not take ownership
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

// references don't take ownership
// borrowing the value but not actually taking ownership
// references are immutable by default

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// you can only have one mutable reference to a piece of data in a particular scope
// this removes the possibility of race conditions

/* 
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // you cannot borrow 's' as mutable more than once at a time

    //prevents data races at compile time
    // data race is two pointers pointing at the same data, and there is no safety
    // ex. one pointer will read the data, another will change it, the pointer that read the data now has incorrect values

    println!("{} {}", r1, r2);
}
*/

/* 
fn main() {
    let mut s = String::from("hello");

    //you can although have multiple/unlimited amount of immutable references
    // becase the underlying data will not change
    let r1 = &s;
    let r2 = &s;    

    // you cannoth ave a mutable reference, if an immutable reference already exists
    // immutable references do not expect the underlying data to change
    //let r3 = &mut s;

    println!("{} {}", r1, r2);

    // at this point r1 and r2 are out of scope
    let r3 = &mut s; // this works since the scope of r1 and r2 ends at the println! statement and has therefore been freed
}
*/

fn main() {
    let reference_to_nothing = dangle();
}

// passing a reference that will be deallocated after the dangle() function has been ran
// therefore rust prevents that from happening
fn dangle() -> &String {
    let s = String::from("hello");

    &s // this reference will be pointing to invalid memory

    // 'this function's return type contains a borrowed value, but there is not value for it to be borrowed from'
}

/*
RULES OF REFERENCES
1. At any given type, you can have one mutable references or any number of immutable references

2. References must always be valid
*/
