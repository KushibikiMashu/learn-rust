use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panicではなくResult型だとエラーであることを明確にできる
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// 返り値に()を使うことは、 runを副作用のためだけに呼び出していると示唆する慣習的な方法です
pub fn run(config: Config) -> Result<(), Box<Error>> {
    // ファイルが見つからない以外に、開く権限がない可能性もある
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
