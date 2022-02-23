use chrono::Local;
pub fn info(msg: &str) {
    println!("{}\tINFO\t{}", Local::now(), msg);
}

pub mod timer {
    use super::*;

    pub fn v1(name: &str, f: Box<dyn Fn(Box<dyn Fn(&str) -> ()>) -> ()>) {
        let start = Local::now();
        info(&format!("TIMER\t[{}]\tTimer Begin: {}", name, start));
        f(Box::new(|msg| info(&format!("TIMER\t{}", msg))));
        let end = Local::now();
        info(&format!("TIMER\t[{}]\tTimer End  : {}", name, start));
        info(&format!("TIMER\t[{}]\tDuration   : {}", name, end - start));
    }
}
