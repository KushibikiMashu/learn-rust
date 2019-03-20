fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // ハッシュマップはKey/Valueで構成される
    // Key, Valueはそれぞれ同じ型で構成される必要がある
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 値を取得する
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // forループを使用したKey/Valueの取得
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Key/Valueの上書き
    scores.insert(String::from("Blue"), 25);
    // {"Blue": 25, "Yellow": 50}
    println!("{:?}", scores);

    // Keyが既存かチェックして、なければKey/Valueを挿入する
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    // {"Blue": 25, "Red": 50, "Yellow": 50}
    println!("{:?}", scores);

    // collectメソッドでHashMapを作成する
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // ハッシュマップは値の所有者になる
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 単語数をチェックする
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // {"hello": 1, "world": 2, "wonderful": 1}
    println!("{:?}", map);
}
