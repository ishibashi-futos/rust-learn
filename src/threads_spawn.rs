use crate::logger;
use std::thread;
use std::time::Duration;

pub fn threads_spawn() {
    let handle = thread::spawn(|| {
        logger::timer::v2(
            "thread_spawn",
            Box::new(|logger| {
                for i in 1..10 {
                    logger(&format!("hi number {} from the spawned thread!", i));
                    thread::sleep(Duration::from_millis(1));
                }
            }),
        );
    });

    for i in 1..5 {
        logger::info(&format!("hi number {} from the main thread!", i));
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // Moveする事でthread内の処理に値を借用させることができる
    let handle = thread::spawn(move || {
        logger::info(&format!("Here's a vector: {:?}", v));
    });

    // drop(v); // vはThreadに借用されているのでdropできない

    handle.join().unwrap();
}
