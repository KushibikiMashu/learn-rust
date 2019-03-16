fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let str = String::from("hello");

    takes_ownership(str);

//    下記エラーが出る
//    error[E0382]: borrow of moved value: `str`
//    println!("{}", str);

    let x = 5;

    makes_copy(x);

    let str1 = gives_ownership();

    let str2 = String::from("hello");

    let str3 = takes_and_gives_back(str2);

    println!("str1 = {}, str2 = {}", str1, str3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}