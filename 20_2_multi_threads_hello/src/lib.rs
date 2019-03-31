use std::{
    thread,
    sync::{
        mpsc,
        Arc,
        Mutex,
    },
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

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

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // receiverに対してlockを呼び出してミューテックスを獲得する
                // ミューテックスのロックを獲得できたら、recvを呼び出してチャンネルからJobを受け取ります
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}

// ThreadPoolはチャンネルを生成し、チャンネルの送信側に就く。
// Workerそれぞれは、チャンネルの受信側に就く。
// チャンネルに送信したいクロージャを保持する新しいJob構造体を生成する。