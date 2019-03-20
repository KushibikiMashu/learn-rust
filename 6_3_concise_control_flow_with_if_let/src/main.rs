#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Hawaii,
    // etc.
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;

//    下記のif let式と等価
//    match coin {
//        Coin:: Quarter(state) => println!("State quarter from {:?}", state),
//        _ => count +=1,
//    }

    let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    }  else {
        count += 1;
    }
}
