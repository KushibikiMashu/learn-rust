use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

// Dropトレイトは、初期化処理に含まれるので、インポートする必要はありません。
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // 変数は、生成されたのと逆の順序でドロップされるので、dはcより先にドロップされます
    let c  = CustomSmartPointer{data: String::from("my stuff")};
    println!("CustomSmartPointer created.");

    // スコープが終わる前に値を強制的にドロップさせる
    drop(c);

    // これは二重解放エラーになる
    // c.drop();
    println!("CustomSmartPointer dropped before the end of main.");

    let d  = CustomSmartPointer{data: String::from("other stuff")};
}
