// Error: recursive type has infinite size
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // ヒープに単独の値を置くことはあまり有用ではないので、このように単独でボックスを使用することはあまりありません。
    // 単独のi32のような値を規定で格納される場所であるスタックに置くことが、大多数の場合にはより適切です。
    // ボックスがなかったら定義することの叶わない型をボックスが定義させてくれる場合を見ましょう。
    let b = Box::new(5);
    println!("b = {}", b);

//    let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}
