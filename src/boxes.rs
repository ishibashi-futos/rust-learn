pub mod boxes {
    use std::ops::Deref;

    use crate::logger;

    pub fn boxes() {
        let b = Box::new(5);

        logger::info(&format!("b: {}", b));

        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        logger::info(&format!("list: {:?}", list));
        // Box<T>がスコープを抜けると、ボックスとそのヒープデータが解放される

        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
        // assert_eq!(5, y); // yは`&i32`なので、`*y`ではなく`y`を使うことはできない

        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // 参照外しをすることで、unboxingすることができる

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // Derefトレイトが実装されていない場合コンパイルできない
                           // 実際には以下のようなコードとして解釈されている
                           // *(y.deref()) // deref関数は&Tを返し、&Tに対して参照外し`*`を適用する

        // Derefトレイトがない場合、コンパイラは`&`参照しか参照外しができない

        let m = MyBox::new(String::from("Rust"));
        hello(&m); // Derefが実装されているので、`&MyBox<String>`を`&str`に暗黙的に変換する

        // Derefが実装されていない場合、以下のように書く必要がある（めんどくさい！）
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]); // `&String`を`&str`に変換する

        // Derefが挿入される回数はコンパイル時に決定するため、実行時のオーバーヘッドはない
    }

    #[derive(Debug)]
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    pub fn hello(name: &str) {
        logger::info(&format!("Hello, {}!", name));
    }
}
