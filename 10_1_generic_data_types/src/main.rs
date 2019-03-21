fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// largest_i32, largest_charを抽象化する
//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
////    この部分はトレイトを使わないとコンパイルできない
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

// ジェネリックな構造体
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Point<f32>にのみ適用されるメソッド
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MultiPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MultiPoint<T, U> {
    fn mixup<V, W>(self, other: MultiPoint<V, W>) -> MultiPoint<T, W> {
        MultiPoint {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['m', 'y', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

//    Point<T>では異なる型を持つとコンパイル不可
//    Point<T, U>で定義する
//    let integer_and_float = Point { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());


    let p1 = MultiPoint { x: 5, y: 10.4 };
    let p2 = MultiPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
