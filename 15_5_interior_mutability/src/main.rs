// 内部可変性は、そのデータへの不変参照がある時でさえもデータを可変化できるRustでのデザインパターンです
// 普通、この行動は借用規則により許可されません。データを可変化するために、このパターンは、データ構造内でunsafeコードを使用して、
// 可変性と借用を支配するRustの通常の規則を捻じ曲げています

// 参照とBox<T>では、借用規則の不変条件は、コンパイル時に強制されています。
// RefCell<T>では、これらの不変条件は、実行時に強制されます。

// Rc<T>は、同じデータに複数の所有者を持たせてくれる。 Box<T>とRefCell<T>は単独の所有者。
// Box<T>は、不変または可変借用をコンパイル時に精査してくれる。Rc<T>は不変借用のみをコンパイル時に精査してくれる。RefCell<T>は、不変または可変借用を実行時に精査してくれる。
// RefCell<T>は実行時に精査される可変借用を許可するので、RefCell<T>が不変でも、RefCell<T>内の値を可変化できる。


// 内部可変性のユースケース: モックオブジェクト
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    //  LimitTrackerのset_valueメソッドの振る舞いをテストしたいです。
//  value引数に渡すものを変えることができますが、set_valueはアサートを行えるものは何も返してくれません。
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            // 警告: 割り当ての75％以上を使用してしまいました
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            // 切迫した警告: 割り当ての90%以上を使用してしまいました
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            // エラー: 割り当てを超えています
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

//            下記コードの場合は実行時にpanicを起こす
//            let mut one_borrow = self.sent_messages.borrow_mut();
//            let mut two_borrow = self.sent_messages.borrow_mut();
//
//            one_borrow.push(String::from(message));
//            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Rc<T>とRefCell<T>を組み合わせることで可変なデータに複数の所有者を持たせる
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::{
    rc::Rc,
    cell::RefCell,
};

fn main() {
    let x = 5;

//    コンパイル不可
//    let y = &mut x;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() +=10;

    println!("a after {:?}", a);
    println!("b after {:?}", b);
    println!("c after {:?}", c);

    // RefCell<T>を使用することで表面上は不変なList値を持てます。
    // しかし、内部可変性へのアクセスを提供するRefCell<T>のメソッドを使用できるので、必要な時にはデータを変更できます。
    // a after Cons(RefCell { value: 15 }, Nil)
    // b after Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    // c after Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}
