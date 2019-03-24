use std::{
    thread,
    time::Duration,
};

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // クロージャが使われてる箇所は限定的なため、型注釈はなくても良い
    // ただし、下記コードでは2回目の呼び出し時にエラーが出る
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    let mut expensive_closure = Cacher::new(|num: u32| -> u32{
        println!("calculating slowlu...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushes!",
                 expensive_closure.value(intensity)
        );

        println!("Next, do {} situps!",
                 expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );

    // 環境をキャプチャする
    // （この場合は、変数xをclosure内で保持しておくこと）
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
