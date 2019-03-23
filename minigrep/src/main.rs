use std::env;

fn main() {
    // Rustにおいて、 型を注釈しなければならない頻度は非常に少ないのですが、
    // collectはよく確かに注釈が必要になる一つの関数です。
    // コンパイラには、あなたが欲しているコレクションの種類が推論できないからです。
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
