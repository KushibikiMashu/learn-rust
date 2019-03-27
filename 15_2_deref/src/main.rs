use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    // derefメソッドの本体を&self.0で埋めているので、
    // derefは*演算子でアクセスしたい値への参照を返します
    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name:&str){
    println!("Hello, {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

//    リスト15-9に*yを入力した時、水面下でコンパイラは、実際にはこのようなコードを走らせていました:
//    *(y.deref())
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
