#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // if needs to evaluate to a boolean value
    // match can be any type
    match coin {
        // match arms
        // pattern/expression => some code
        // when match expression executes, it compare value against pattern of each arm
        Coin::Penny => {
            // runs both the print and returns 1 afterwards
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // when a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // patterns must be exhaustive
    // ex. rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Value in cents: {}", value_in_cents(Coin::Dime));
    println!("Value in cents: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five plus one: {}", six);

    let dice_roll = 9;
    // have to put the catch-all arm last because the patterns are evaluated in order
    // if we put the catch-all arm earlier, the other arms would never run
    /* 
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    */

    //catch all but _ is used when we do not need to use the value
    // tells rust that we aren't going to use that variable
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // nothing else happens
        //_ => reroll(),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

