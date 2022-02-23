pub mod reference_counting {
    use std::rc::Rc;

    use crate::logger;

    pub fn reference_counting() {
        logger::timer::v1(
            "RC",
            Box::new(|logger| {
                let a = BoxList::Cons(
                    1,
                    Box::new(BoxList::Cons(
                        2,
                        Box::new(BoxList::Cons(3, Box::new(BoxList::Nil))),
                    )),
                );
                #[allow(unused_variables)]
                let b = BoxList::Cons(3, Box::new(a));
                // let c = RCList::Cons(4, Box::new(a)); // `a`はmoveされているため作成できない

                let a = Rc::new(RCList::Cons(
                    1,
                    Rc::new(RCList::Cons(
                        2,
                        Rc::new(RCList::Cons(3, Rc::new(RCList::Nil))),
                    )),
                ));
                logger(&format!("a: {}", Rc::strong_count(&a)));
                #[allow(unused_variables)]
                let b = RCList::Cons(3, Rc::clone(&a));
                logger(&format!("a: {}", Rc::strong_count(&a)));
                #[allow(unused_variables)]
                let c = RCList::Cons(4, Rc::clone(&a)); // Rc::cloneで値をクローンするたびに参照カウントが増える
                logger(&format!("a: {}", Rc::strong_count(&a)));
                {
                    #[allow(unused_variables)]
                    let b = RCList::Cons(3, Rc::clone(&a));
                    logger(&format!("a: {}", Rc::strong_count(&a)));
                    #[allow(unused_variables)]
                    let c = RCList::Cons(4, Rc::clone(&a));
                    logger(&format!("a: {}", Rc::strong_count(&a)));
                    // ブロックが終了する際に`c`, `b`が破棄されてカウントが減る
                }
                drop(b);
                logger(&format!("a: {}", Rc::strong_count(&a))); // Dropしてもカウントは減る
                drop(c);
                logger(&format!("a: {}", Rc::strong_count(&a)));
                drop(a);
                // logger(&format!("a: {}", Rc::strong_count(&a))); // aは破棄されているためエラーになる
            }),
        );
    }

    pub enum BoxList {
        Cons(i32, Box<BoxList>),
        Nil,
    }

    pub enum RCList {
        Cons(i32, Rc<RCList>),
        Nil,
    }
}
