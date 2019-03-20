pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 全ての名前をスコープに導入する
use TrafficLight::*;

fn main() {
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
