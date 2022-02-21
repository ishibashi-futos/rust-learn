use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

use crate::logger;

pub fn closures() {
    simulated_expensive_calculation(100);

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // クロージャにおいては、型注釈は必須ではない
    let expensive_closure = |num| {
        logger::info("[closure]calculating slowly...");
        thread::sleep(Duration::from_millis(200));
        num
    };

    let num = expensive_closure(5);
    logger::info(&format!("[closure]num: {}", num));

    let num = expensive_closure(10);
    logger::info(&format!("[closure]num: {}", num));

    // 以下の関数は概ね等価である(使われ方によって型が決まる)
    fn add_one_v1(x: i32) -> i32 {
        x + 1
    }
    logger::info(&format!("add_one_v1: {}", add_one_v1(1)));
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    logger::info(&format!("add_one_v2: {}", add_one_v2(1)));
    let add_one_v3 = |x| x + 1;
    logger::info(&format!("add_one_v1: {}", add_one_v3(1)));
    let add_one_v4 = |x| x + 1;
    logger::info(&format!("add_one_v1: {}", add_one_v4(1)));

    let example_closure = |x| x;

    // 入出力の型が一致するうちはコンパイルエラーにならない
    let s = example_closure(String::from("hello"));
    logger::info(&format!("example_closure: {}", s));
    let s = example_closure(String::from("world"));
    logger::info(&format!("example_closure: {}", s));

    // Stringで呼び出した時点で型が決定されているので、float型ではコンパイルエラーになる
    // let n = example_closure(1.5);

    generate_workout(10, 3);
    generate_workout(24, 5);
    generate_workout(25, 5);
    generate_workout(25, 3);

    let x = 4;
    // クロージャの場合は変数xのスコープにある変数も参照できる
    let equal_to_x = |z| z == x;
    // 関数の場合はスコープが異なるので変数xは参照できない
    // fn equal_to_x(z: i32) -> bool { z == x }
    logger::info(&format!("equal_to_x: {}", equal_to_x(5)));
    logger::info(&format!("equal_to_x: {}", equal_to_x(4)));

    // FnOnceは、クロージャの環境にあるスコープからキャプチャした変数を消費し、Moveしてしまう（所有権を奪う）
    // FnMutは、可変で値を使用するので、変数を変更できる
    // Fnは環境から変数を普遍の状態で借用します
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    logger::info("calculating slowly...");
    thread::sleep(Duration::from_millis(200));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    let mut expensive_result = Cacher::new(|num| {
        logger::info("[closure]calculating slowly...");
        thread::sleep(Duration::from_millis(200));
        num
    });

    if intensity < 25 {
        logger::info(&format!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        ));

        logger::info(&format!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        ));
    } else {
        if random_number == 3 {
            logger::info("Taks a break today! Remember to stay hydrated!");
        } else {
            logger::info(&format!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            ));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                logger::info(&format!("[closure]cached value: {}", v));
                v
            }
            None => {
                logger::info("[closure]value not found");
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub struct MappedCache<T, R>
where
    T: Fn(R) -> R,
{
    calculation: T,
    values: HashMap<R, R>,
}

impl<T, R: Eq + Hash + Copy> MappedCache<T, R>
where
    T: Fn(R) -> R,
{
    #[allow(dead_code)]
    pub fn new(calculation: T) -> MappedCache<T, R> {
        MappedCache {
            calculation,
            values: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn value(&mut self, arg: R) -> R {
        let v = self.values.entry(arg).or_insert((self.calculation)(arg));
        *v
    }

    #[allow(dead_code)]
    pub fn value_or_default<F: Fn(R) -> R>(&mut self, arg: R, generator: F) -> R {
        let v = self.values.entry(arg).or_insert(generator(arg));
        *v
    }
}
