pub mod boxes {
    use crate::logger;

    pub fn boxes() {
        let b = Box::new(5);

        logger::info(&format!("b: {}", b));

        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

        logger::info(&format!("list: {:?}", list));
    }

    #[derive(Debug)]
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }
}
