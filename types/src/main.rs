fn main() {
    let guess: u32 = "32".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'z';

    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of b is: {}", b);

    let t: (i32, f64, u8) = (a, b, c);

    let five_hundred = t.0;

    let six_point_four = t.1;

    let one = t.2;

    let array = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let first = array[0];

    let second= array[1];
}