use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // key, value
    // we are passing the ownership not only as reference
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    // returns an optional value, since the key might be invalid
    let score = scores.get(&team_name);

    // extract a tuple with the key and value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // overwrites the value in the blue key

    // if Yellow key does not exist, insert 30. If it does, do not overwrite
    scores.entry(String::from("Yellow")).or_insert(30); // create a new value for yellow
    scores.entry(String::from("Yellow")).or_insert(40); // since yellow exits, do nothing


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // split all the words by whitespace
    // ["hello", "world", "wonderful", "world"]
    for word in text.split_whitespace() {
        // check if word is already in map, if not set its value to 0
        // or_insert returns a mutable reference to the value
        let count = map.entry(word).or_insert(0);
        
        // dereference the count value and increment on the number count of the word
        *count += 1;
    }
    
    println!("{:?}", map);
}