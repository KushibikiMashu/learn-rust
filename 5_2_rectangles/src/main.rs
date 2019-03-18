#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
//    タテヨコを変数で定義する
//    let width1 = 30;
//    let height1 = 50;

//    タテヨコをタプルで定義する
//    let rect1 = (30, 50);

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
}

//  タテヨコを変数で定義するときの関数
//  fn area(width: u32, height: u32) -> u32 {
//    width * height
//  }

//  タテヨコをタプルで定義するときの関数
//  fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//  }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
