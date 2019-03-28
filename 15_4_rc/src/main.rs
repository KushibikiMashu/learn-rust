// 複数の所有権を可能にするRc<T>型（reference counting型）
//  Rc<T>型は、値がまだ使用中かどうか決定する値への参照の数を追跡します。
//  値への参照が0なら、どの参照も無効にすることなく、値は片付けられます。
// ヒープにプログラムの複数箇所で読む何らかのデータを確保したい時にRc<T>型を使用する

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // 参照カウント
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Rc::cloneの実装は、多くの型のclone実装のように、全てのデータのディープコピーをすることではありません。
    // Rc::cloneの呼び出しは、参照カウントをインクリメントするだけであり、時間はかかりません
    let b = Cons(3, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));

        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2
