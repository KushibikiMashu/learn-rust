fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            // 紫を背景色に使用します
            println!("Using purple as the background color");
        } else {
            // オレンジを背景色に使用します
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // if let式を使うことの欠点は、コンパイラが網羅性を確認してくれないことです。一方でmatch式ではしてくれます。
    // 最後のelseブロックを省略して故に、扱い忘れたケースがあっても、コンパイラは、ロジックバグの可能性を指摘してくれないでしょう。

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top)=stack.pop(){
        println!("{}", top);
    }

    let v = vec!['a','b','c'];

    // enumerateの最初の呼び出しは、タプル(0, 'a')を生成します
    for (index, value)in v.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }

    let point=(3,5);
    print_coordinates(&point);
}

fn print_coordinates(&(x,y): &(i32,i32)){
    println!("Current location: ({}, {})", x, y);
}