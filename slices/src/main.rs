// slicing -> let you reference a contiguous sequence of elements within a collection rather than referencing the entire collection
// does not take ownership of the underlying data

/* 
fn main() {
    let mut s = String::from("Hello world!");
    //s.clear(); 
    // now our first_word variable is still 5 even though it is now empty
    // return value is not tied to the string itself. You have to manually keep the return value in sync with the string

    // string slices
    // give us a reference to the string, but we only want part of the string

    let hello = &s[0..5]; // start from index 5, 5 is exclusive
    let world = &s[6..11]; // start from index 6, 11 is exclusive

    let hello = &s[..5]; // start can be ommited, start to index 5
    let world = &s[6..]; // end can be ommited, 6 to last index

    let whole_word = &s[..]; //get the whole string

    let first_word = first_word(&s);
    
    s.clear();
    println!("{}", first_word);
}
*/

/*
// first implemented without string slices
// uses references because we do not need to take ownership of the string
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter and enumerate in order to the get index and the item itself
    for (i, &item) in bytes.iter().enumerate() {
        //check if empty space which signifies the end of a word
        if item == b' ' {
            return i;
        }
    }

    // return the length of the string if it is one word only
    s.len()
}
*/

//&str is the type for string slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // iter and enumerate in order to the get index and the item itself
    for (i, &item) in bytes.iter().enumerate() {
        //check if empty space which signifies the end of a word
        if item == b' ' {
            // return a string slice from the beginning to the end
            return &s[0..i];
        }
    }

    // return the length of the string if it is one word only
    &s[..]
}

// string literals are string slices
// the same function of you need to use it for string literals
/*
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // iter and enumerate in order to the get index and the item itself
    for (i, &item) in bytes.iter().enumerate() {
        //check if empty space which signifies the end of a word
        if item == b' ' {
            // return a string slice from the beginning to the end
            return &s[0..i];
        }
    }

    // return the length of the string if it is one word only
    &s[..]
}
*/

fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[0..2];
}