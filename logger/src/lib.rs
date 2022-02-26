pub mod spinners;

pub mod logger {
    use chrono::Local;

    pub fn info(msg: &str) {
        println!("{}\tINFO\t{}", Local::now(), msg);
    }

    pub mod timer {
        use super::*;

        #[allow(dead_code)]
        pub fn v1(name: &str, f: Box<dyn Fn(Box<dyn Fn(&str) -> ()>) -> ()>) {
            let start = Local::now();
            info(&format!("TIMER\t[{}]\tTimer Begin: {}", name, start));
            f(Box::new(|msg| info(&format!("TIMER\t{}", msg))));
            let end = Local::now();
            info(&format!("TIMER\t[{}]\tTimer End  : {}", name, end));
            info(&format!("TIMER\t[{}]\tDuration   : {}", name, end - start));
        }

        #[allow(dead_code)]
        pub fn v2(name: &'static str, f: Box<dyn Fn(Box<dyn Fn(&str) -> ()>)>) {
            let start = Local::now();
            info(&format!("TIMER\t[{}]\tTimer Begin: {}", name, start));
            // captureした値をmoveする事で、メンバ関数内に実態として値をキャプチャできる（参照ではないのでライフタイムが関係ない）
            f(Box::new(move |msg| {
                info(&format!("TIMER\t[{}]\t{}", name, msg))
            }));
            let end = Local::now();
            info(&format!("TIMER\t[{}]\tTimer End  : {}", name, end));
            info(&format!("TIMER\t[{}]\tDuration   : {}", name, end - start));
        }
    }

    pub mod spinner {
        use std::{
            io::{stdout, Write},
            sync::mpsc,
            thread::JoinHandle,
            time::Duration,
        };

        use crate::spinners::spinners;

        use super::*;

        pub fn v1(name: &'static str, f: Box<dyn Fn() -> () + Send>) {
            let start = Local::now();
            info(&format!("TIMER\t[{}]\tTimer Begin: {}", name, start));
            let (sender, reciever) = mpsc::channel();
            let _ = std::thread::spawn(move || {
                f();
                let end = Local::now();
                sender.send(end)
            });

            let handler = do_spinner(name);

            let end = reciever.recv().unwrap(); // fnが終了するのを待つ

            drop(handler); // dropする事で強制的にスレッドを終了させる
            print!("\r"); // Spinnerを消す
            info(&format!("TIMER\t[{}]\tTimer End  : {}", name, end));
            info(&format!("TIMER\t[{}]\tDuration   : {}", name, end - start));
        }

        fn do_spinner(name: &'static str) -> JoinHandle<()> {
            std::thread::spawn(move || {
                let mut spinner = spinners::Dots::new();
                // let mut spinner = spinners::Moon::new();
                // let mut spinner = spinners::Clock::new();
                let mut stdout = stdout();
                loop {
                    print!("\r{} Processing - [{}]", spinner.next().unwrap(), name);
                    stdout.flush().unwrap();
                    std::thread::sleep(Duration::from_millis(100));
                }
            })
        }
    }
}
