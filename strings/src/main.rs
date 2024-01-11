use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new(); // initialize an empty string
    let s2 = "initial contents"; // string slice
    let s3 = s2.to_string(); // convert a string slice to a string
    let s4 = String::from("initial contents"); // create a string from a string slice

    // strings can grow or shrink in size
    let mut s = String::from("foo");
    s.push_str("bar"); // takes a string slice since we don't want to take ownership of the string being passed in
    s.push('!');

    // appending strings using the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 + a reference to s2
    // taking ownership of s1 to s3 then appending characters of s2 to s3
    // let s3:String = s1 + &s2;

    // since we moved ownership of s1, we cannot use it afterwards
    // we cannot borrow a value after it has been moved
    // println!("{}", s1);

    // appending using the format macro
    // format macro does not taking ownership of these strings
    let s3 = format!("{}{}",s1 ,s2);
    println!("{}", s3);

    // indexing in strings
    let hello = String::from("Hello");

    // strings cannot be indexed by integers
    // UTF-8 characters can range in the size that they are from 1 to 4 bytes
    // unlike ASCII where all of the values are 1 byte
    // let c:char = hello[0];

    // iterate thrugh the string using bytes
    for byte in "hello".bytes() {
        println!("{}", byte);
    }

    // scalar values
    for char in "hello".chars() {
        println!("{}", char);
    }
    
    // grapheme clusters
    for graphene in "hello".graphemes(true) {
        println!("{}", graphene);
    }
}