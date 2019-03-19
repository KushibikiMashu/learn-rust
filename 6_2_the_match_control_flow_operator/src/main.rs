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

fn value_in_cents(coin: Coin) -> u32 {
    // enumとmatch式の組み合わせは強力
    // match式では、=>演算子を挟んで左はマッチするパターン、右は式を書く
    match coin {
        // 式を複数行書く場合は、{}で囲む
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // match式ではNoneを必ず定義する
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // プレースホルダー
    let some_u8_value = 5;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // 1,3,5,7以外の値にマッチする
        _ => (),
    }
}
