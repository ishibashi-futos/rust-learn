use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub mod reference_cycles {
    use std::{
        cell::RefCell,
        rc::{Rc, Weak},
    };

    #[derive(Debug)]
    pub enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match *self {
                List::Cons(_, ref item) => Some(item),
                List::Nil => None,
            }
        }
    }

    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
        pub parent: RefCell<Weak<Node>>,
        pub children: RefCell<Vec<Rc<Node>>>,
    }
}

use reference_cycles::List::{Cons, Nil};
use reference_cycles::Node;

use crate::logger;

pub fn reference_cycles() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    logger::info(&format!("a initial rc count = {}", Rc::strong_count(&a)));
    logger::info(&format!("a next item = {:?}", a.tail()));

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // aの値を借り受ける

    logger::info(&format!(
        "a rc count after b creation: = {}",
        Rc::strong_count(&a)
    )); // aは2つのrcがある
    logger::info(&format!("b initial rc count = {}", Rc::strong_count(&b)));
    logger::info(&format!("b next item = {:?}", b.tail()));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // aのtailをbにする
    }

    logger::info(&format!(
        "b rc count after changing a: = {}",
        Rc::strong_count(&b)
    ));
    logger::info(&format!(
        "a rc count after changing a: = {}",
        Rc::strong_count(&a)
    ));

    // 以下の行をコメントアウトすると、スタックオーバーフローになる
    // logger::info(&format!("a next item = {:?}", a.tail())); // stack overflow!

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    logger::info(&format!("leaf: {:?}", &leaf));
    logger::info(&format!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    )); // Parentは登録されていないのでこの時点ではNone
    logger::info(&format!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    ));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        logger::info(&format!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        ));

        logger::info(&format!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        ));
    }

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    logger::info(&format!("leaf: {:?}", &branch));

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // leafのparentをbranchにする
    logger::info(&format!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    )); // この場合はbranchがparentになる
}
