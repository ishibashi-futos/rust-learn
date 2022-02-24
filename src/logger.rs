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
