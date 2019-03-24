extern crate minigrep;

use std::{
    env,
    process,
};

use minigrep::Config;

// プログラムをmain.rsとlib.rsに分け、ロジックをlib.rsに移動する。
// コマンドライン引数の解析ロジックが小規模な限り、main.rsに置いても良い。
// コマンドライン引数の解析ロジックが複雑化の様相を呈し始めたら、main.rsから抽出してlib.rsに移動する。
fn main() {
    // Rustにおいて、 型を注釈しなければならない頻度は非常に少ないのですが、
    // collectはよく確かに注釈が必要になる一つの関数です。
    // コンパイラには、あなたが欲しているコレクションの種類が推論できないからです。
    let args: Vec<String> = env::args().collect();

    // プログラムの設定用変数のパート
    // unwrap_or_elseは、値がOkならOkの値を返し、
    // 値がErr値なら、クロージャにErrの値を引数として渡す
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run(config)はプログラムのロジックのパート
    // run(config)はOkの時に値を返さないので、
    // unwrap_or_elseを使わず、if let式でエラーの処理のみを書く
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
