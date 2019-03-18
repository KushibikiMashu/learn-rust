fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let string = no_dangle();
}

fn calculate_length(s: &String)-> usize{
    s.len()
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}