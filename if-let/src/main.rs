fn main() {
    let config_max = Some(3u8);
    // only does something when config max is not none
    // _ => () adds additional boilerplate that we do not need
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // does the same as the match expression above
    // pattern = expression
    // only takes care of a single pattern and ignores the others
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    /* 
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    */

    // also has an else function but if you might want to use the match instead
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
