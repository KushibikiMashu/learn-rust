fn main() {
    let s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // StringはUTF-8でエンコードされている
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    println!("{}", hello);

    let mut str = String::from("foo");

    // 文字列スライスを追加
    str.push_str("bar");

    println!("{}", str);

    // 所有権を取得する必要はない
    let mut s1: String = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // pushメソッドは単一の文字をパラメータとして取る
    s1.push('z');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1の所有権はs3に映るため、これ以降利用できない

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 文字列の連結は扱いにくい
//    let s = s1 + "-" + &s2 + "-" + &s3;

    // format!マクロを用いる
    // format!は所有権を取得しない
    let s = format!("{}-{}-{}", s1,s2,s3);

    // エラーが発生する
    // s1 = String::from("hello");
    // h = s1[0];

    // Stringはvec<u8>のラッパー
    // 値は4
    let len = String::from("Hola").len();

    // 値は24
    let len = String::from("Здравствуйте").len();

    let hello = "Здравствуйте";

    // 値はЗд。だが、一文字2バイトということを知っていなければならない
    let s = &hello[0..4];

    let hindi ="नमस्ते";

    // 文字列の要素にアクセスする
    for c in hindi.chars(){
        println!("{}", c);
    }

    // バイトを取得する
    for b in hindi.bytes() {
        println!("{}", b);
    }
}
