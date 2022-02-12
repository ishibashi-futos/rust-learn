use chrono::{Local};
pub fn info(msg: &str) {
    println!("{}\tINFO\t{}", Local::now(), msg);
}
