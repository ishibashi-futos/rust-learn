use std::sync::{Arc, Mutex}; // 代わりにArcを使用する
                             // use std::rc::Rc; // Rcはスレッドで値を共有するために安全ではないのでコンパイルエラーになる
use std::thread;

use crate::logger;

pub fn shared_state() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    logger::info(&format!("m = {:?}", m));

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            logger::info(&format!("thread {:?}: {}", thread::current().id(), *num));
            *num += 1;
            // デストラクタで暗黙的にアンロックされている？
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    logger::info(&format!("Result: {}", *counter.lock().unwrap()));
}
