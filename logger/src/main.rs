use std::time::Duration;

extern crate logger;

fn main() {
    logger::logger::spinner::v1("spinner", Box::new(|| {
        for _ in 0..=100 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }));

}
