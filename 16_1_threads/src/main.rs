use std::{
    thread,
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join()呼び出しのためにメインスレッドは待機し、
    // 立ち上げたスレッドが終了するまで終わりません。
    handle.join().unwrap();


    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // println!はvへの参照のみを必要とするので、クロージャは、vを借用しようとします。ですが、問題があります。
    // コンパイラには、立ち上げたスレッドがどのくらいの期間走るのかわからないので、vへの参照が常に有効であるか把握できないのです。

    // クロージャの前にmoveキーワードを付することで、コンパイラに値を借用すべきと推論させるのではなく、
    // クロージャに使用している値の所有権を強制的に奪わせます
    let handle2 = thread::spawn(move|| {
       println!("Here's a vector: {:?}", v);
    });

    drop(v);

    handle2.join().unwrap();

}
