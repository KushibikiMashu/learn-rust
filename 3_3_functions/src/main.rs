fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let x = 6; // 文(statement)

    // 式
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let five = five();

    println!("The value of five is: {}", five);

    let six = plus_one(five);

    println!("The value of six is: {}", six);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}