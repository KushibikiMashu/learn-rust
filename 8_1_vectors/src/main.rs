enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// ベクトルの作成、更新、値の取得をする
fn main() {
//    let v ; Vec<i32> = Vec::new();

//    Rustはvecの肩を推測する
//    let v = vec![1,2,3];

//  ベクトルの値はヒープ領域に格納される
    let mut v = Vec::new();

    // ベクトルは同じ型の値しか格納できない
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

//    let does_noe_exist = &v[100];
//    let does_noe_exist = v.get(100);

//    errorが発生する
//    let first = &v[0];
//    v.push(6);
//    println!("The first element is: {}", first);

    for i in &v {
        println!("{}", i);
    }

    // 間接参照演算子 * を利用して参照する値を更新する
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match v.pop() {
        Some(55) => println!("The last element is {}", 5),
        _ => println!("Other"),
        None => println!("There is no third element."),
    }
}
