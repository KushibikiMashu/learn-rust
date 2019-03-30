// プログラミングにおけるチャンネルは、2分割できます: 転送機と受信機です。
// 転送機はアヒルのおもちゃを川に置く上流になり、受信機は、アヒルのおもちゃが行き着く下流になります。
// コードのある箇所が送信したいデータとともに転送機のメソッドを呼び出し、 別の部分がメッセージが到着していないか受信側を調べます。
// 転送機と受信機のどちらかがドロップされると、 チャンネルは閉じられたと言います。

// 1つのスレッドが値を生成し、それをチャンネルに送信し、別のスレッドがその値を受け取り、出力するプログラムに取り掛かります。
// チャンネルを使用してスレッド間に単純な値を送り、 機能の説明を行います。
// 一旦、そのテクニックに慣れてしまえば、チャンネルを使用してチャットシステムや、多くのスレッドが計算の一部を担い、
// 結果をまとめる1つのスレッドにその部分を送るようなシステムを実装できるでしょう

// mpsc::channel関数で新しいチャンネルを生成しています; mpscはmultiple producer, single consumerを表しています
use std::{
    thread,
    sync::mpsc,
    time::Duration,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals=vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // sendメソッドはResult<T, E>型を返すので、既に受信側がドロップされ、
            // 値を送信する場所がなければ、送信処理はエラーを返します
            // send関数は引数の所有権を奪い、 値がムーブされると、受信側が所有権を得る
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let vals=vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // メインスレッドの実行をブロックし、値がチャンネルを流れてくるまで待機します
    // チャンネルの送信側が閉じたら、recvはエラーを返し、もう値は来ないと通知します
    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {}", received);
    }
}

// Got: hi
// Got: more
// Got: from
// Got: messages
// Got: for
// Got: the
// Got: thread
// Got: you