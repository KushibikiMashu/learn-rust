// Box<T>型は、既知のサイズで、ヒープに確保されたデータを指します。
// Rc<T>型は、ヒープのデータへの参照の数を追跡するので、データは複数の所有者を保有できます。
// 内部可変性のあるRefCell<T>型は、不変型が必要だけれども、その型の中の値を変更する必要がある時に使用できる型を与えてくれます。
// また、コンパイル時ではなく実行時に借用規則を強制します。

// 循環の各要素の参照カウントが絶対に0にならないので、これはメモリリークを起こし、値は絶対にドロップされません。
use List::{Cons, Nil};
use std::{
    rc::{Rc, Weak},
    cell::RefCell,
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // 循環参照
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // Rc::cloneを呼び出すとRc<T>インスタンスのstrong_countが増え、
    // strong_countが0になった時にRc<T>インスタンスは片付けられる
    // aの最初の参照カウント = {}
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // aの次の要素は = {:?}
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // b作成後のaの参照カウント = {}
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // bの最初の参照カウント = {}
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // bの次の要素 = {:?}
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // aを変更後のbの参照カウント = {}
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // aを変更後のaの参照カウント = {}
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // a initial rc count = 1
    // a next item = Some(RefCell { value: Nil })
    // a rc count after b creation = 2
    // b initial rc count = 1
    // b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    // b rc count after changing a = 2
    // a rc count after changing a = 2

    // コメントアウトを外すと下記のスタックオーバーフローを起こす
    // println!("a next item = {:?}", a.tail());

    // RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5,
    // RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell {
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // Aborted


    // 木データ構造を作る
    // [branch] → [leaf]
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        // leafのstrong_count = {}, weak_count = {}
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            // branchのstrong_count = {}, weak_count = {}
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[derive(Debug)]
struct Node {
    value: i32,
    // 親ノードがドロップされたら、 子ノードもドロップされるべきなのです。ですが、子供は親を所有するべきではありません
    // 子ノードをドロップしても、親はまだ存在するべきです
    // ここで弱い参照を使います
    // これでノードは親ノードを参照できるものの、所有はしなくなります
    // つまり、Weak（弱い参照）を使うことで、循環参照を回避することができる
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


