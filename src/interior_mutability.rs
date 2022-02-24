use crate::interior_mutability::List::{Cons, Nil};
use rust_learn::logger;

pub fn interior_mutability() {
    logger::timer::v2(
        "interior-mutability",
        Box::new(|logger| {
            // 以下のコードはコンパイルできない
            // 不変に見える変数を可変で借用する事はできない
            // let x = 5;
            // let y = &mut x;

            let messenger = MockMessenger::new();
            let mut t = LimitTracker::new(&messenger, 100);
            t.set_value(80);
            t.set_value(90);
            t.set_value(100);
            logger(&format!("t: {:?}", t));

            let value = Rc::new(RefCell::new(5));
            let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
            let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
            let c = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

            // borrow_mutでRefMutスマートポインタを取得し、`*`で参照外しして値を書き換える
            *value.borrow_mut() += 10;

            logger(&format!("a: {:?}", a));
            logger(&format!("b: {:?}", b)); // 以下のいずれも、`a`と`b`は同じ値を参照しているため、`a`の変更が反映される
            logger(&format!("c: {:?}", c)); // 15になっているはず
        }),
    );
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

#[derive(Debug)]
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            // 警告: 割り当ての75％以上を使用してしまいました
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            // 切迫した警告: 割り当ての90%以上を使用してしまいました
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            // エラー: 割り当てを超えています
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct MockMessenger {
    send_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            send_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.send_messages.borrow_mut().push(String::from(message));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_tests() {
        let messenger = PanicMessenger::new();
        let mut t = LimitTracker::new(&messenger, 100);
        t.set_value(80);
    }

    struct PanicMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl PanicMessenger {
        fn new() -> PanicMessenger {
            PanicMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for PanicMessenger {
        fn send(&self, message: &str) {
            let mut borrowed1 = self.send_messages.borrow_mut();
            let mut borrowed2 = self.send_messages.borrow_mut();

            borrowed1.push(String::from(message));
            borrowed2.push(String::from(message)); // should panic!
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
