use std::{
    thread,
    sync::{
        mpsc,
        Arc,
        Mutex,
    },
};

// Message enumはスレッドが実行すべきJobを保持するNewJob列挙子か、
// スレッドをループから抜けさせ、停止させるTerminate列挙子のどちらかになります
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// ThreadPoolはチャンネルを生成し、チャンネルの送信側に就く。
// Workerそれぞれは、チャンネルの受信側に就く。
// チャンネルに送信したいクロージャを保持する新しいJob構造体を生成する。

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // 複数のスレッドで所有権を共有しつつ、 スレッドに値を可変化させるためには、Arc<Mutex<T>>を使用する必要があります
        // Arc型は、複数のワーカーに受信者を所有させ、Mutexにより、1度に受信者から1つの仕事をたった1つのワーカーが受け取ることを保証します
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // クロージャを受け取る
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                // Optionのtakeメソッドは、Some列挙子を取り出し、その箇所にNoneを残します
                // ワーカーのスレッドが既にNoneなら、ワーカーはスレッドを既に片付け済みであることがわかるので、その場合には何も起きません
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    // 実行中のWorkerにはthreadにSome列挙子があり、Workerを片付けたい時には、
    // ワーカーが実行するスレッドがないようにSomeをNoneで置き換えるのです。
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // receiverに対してlockを呼び出してミューテックスを獲得する
                // ミューテックスのロックを獲得できたら、recvを呼び出してチャンネルからJobを受け取ります
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
