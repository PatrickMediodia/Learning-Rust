fn main() {
    let a = [1, 2, 3];

    // since we don't have initial values, rust cannot infer its type
    // we therefore have to explicitly say its type
    // vectors can only store one type of data at a time
    let mut v1:Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);

    // vec marco allows you to initialize vectors with values
    // no need to tell the type explicitly since rust can infer the type based on our initial values
    let mut v = vec![1, 2, 3, 4, 5];

    /*
    // like any other type, vectors are dropped when they are out of scope
    // in this case, it will be dropped after the {}
    {
        let v2 = vec![1, 2, 3];
    }
    */

    // &v -> reference to the vector
    // problem with this is that you can specify an invalid index
    // this is because we don't know the size of the vector on compile time
    let third = &v[2];
    // v.push(6); would not work since we already have an immutable reference to our vector
    // recall that you cannot have an immutable and mutable reference to an item at the same time
    println!("The third element is {}", third);

    // use the get method to safely/gracefully access items in a vector
    // we use a match expression to handle the Some or None case
    // the .get method returns an enum Option<T>
    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // loop through an immutable reference to each of the items in the vector
    for i in &mut v {
        // used the dereference operator to get the underlying value of the mutable reference
        *i += 1;
    }

    // loop through an immutable reference to each of the items in the vector
    for i in &v {
        println!("{}", i);
    }




    // remember that vectors can only store one type
    // but that type can be enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer!"),
    }
}