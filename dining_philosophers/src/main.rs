use std::{
    thread,
    time::Duration,
    sync::{
        Mutex,
        Arc,
    },
};

struct Philosopher {
    // 名前には&str型ではなくString型を選びました
    // 一般的に、データを所有する型を用いた方が、データを参照する型の利用よりも簡単になります
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        // インデクスから Mutex が得られたら、lock() を呼び出します
        // ミューテックスが別スレッドから並行アクセスされていた場合は、有効になるまでブロックされるでしょう
        // _left と _right がスコープから抜けるとき、ロックは自動的に解放されます
        let _left = table.forks[self.left].lock().unwrap();
        println!("left fork {} is being used.", self.left);
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
        println!("right fork {} is being used.", self.right);

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    // 「arc」は「アトミック参照カウント(atomic reference count)」を意味し、複数スレッドからTableを共有するために必要となります
    // 共有するときは参照カウントを増やし、各スレッドの終了時にはカウントを減らします
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        // 引数は0, 4 としています。これはデッドロックを防ぐためのものです
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    // into_iter()で哲学者の所有権を持つイテレータを生成します
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        // スレッドはそのスレッドを制御するハンドルを返す
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
