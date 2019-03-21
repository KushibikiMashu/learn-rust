use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                }
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // ResultがErr列挙子ならunwrapはpanic!を呼ぶ
    // match式を省略でき、コードが冗長にならずに済む
    let f = FIle::open("hello1.txt").unwrap();

    // expectはエラーメッセージを提供する
    let f = FIle::open("hello2.txt").expect("Faild to open hello2.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?演算子を利用すると、上記と同じ内容の関数がシンプルに書ける
// Resultの値がOkなら、Okの中身がこの式から返ってくる
// Resultの値がErrなら、 returnキーワードを使ったかのように関数全体からErrの中身が返ってくる
// ?演算子はResultを返す関数でしか使用できない
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// さらに短くすることができる
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
