#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッドの引数
// 読み込み専用(&self)
// 書き込みもする(&mut self)
// 所有権を奪う(self)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn change(&mut self) -> Self {
        Self {
            width: 10,
            height: 20,
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("The are of the square is {}", sq.area());

//    構造体の値を変更する
    let mut rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The instance of new rect1 is {:#?}",
        rect1.change()
    );
}
