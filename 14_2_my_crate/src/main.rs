extern crate art;

// 役に立つAPI構造を作ることは、科学というよりも芸術の領域であり、
// ユーザにとって何が最善のAPIなのか、 探求するために繰り返してみることができます。
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}