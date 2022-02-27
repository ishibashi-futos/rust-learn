use std::ops::Add;
extern crate logger;
use logger::logger::spinner;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// std::opsに列挙されたトレイトを実装することで演算子をオーバーロードできる
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
// Addトレイトは以下のようになっている
/// ```rust
/// trait Add<RHS=Self> {
///    type Output;
///    fn add(self, rhs: RHS) -> Self::Output;
/// }
/// ```

trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// ただ定義しただけではDisplay::fmrを実装する必要がある旨エラーが出る
impl OutlinePrint for Point {}

// std::fmt::Displayを実装することで、OutlinePrintトレイトの条件を満たすことができる
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 異なる単位を持つ構造体にも適用できる
#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self) -> &str;
}

trait Wizard {
    fn fly(&self) -> &str;
}

// flyメソッドの実装が3つある
struct Human {}

impl Pilot for Human {
    fn fly(&self) -> &str {
        "This is your captain speaking."
    }
}

impl Wizard for Human {
    fn fly(&self) -> &str {
        "Up"
    }
}

impl Human {
    fn fly(&self) -> &str {
        "*waving arms furiously*"
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        // スポット(Wikipediaによると、飼い主の事故死後もその人の帰りを待つ忠犬の名前の模様)
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        // 子犬
        String::from("puppy")
    }
}

// Vecには既にDisplay関数が実装されており、オーバーライドできないが、
// トレイトか型がクレートローカルである限り、型に既存のトレイトを実装できる
// Vecに独自のトレイトを実装する場合、タプル構造体を使用し薄いラッパーを作成する
struct Wrapper(Vec<String>);
impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn advanced_traits() {
    spinner::v1(
        "advanced trait",
        Box::new(|| {
            let a = Point { x: 1, y: 0 };
            let b = Point { x: 2, y: 3 };

            let c = a + b;

            assert_eq!(c, Point { x: 3, y: 3 });

            let mills = Millimeters(1000);
            let meters = Meters(1);

            let c = mills + meters;

            assert_eq!(c, Millimeters(2000));

            let person = Human {};
            // どの実装を呼び出すか明示的に指定する
            assert_eq!("This is your captain speaking.", Pilot::fly(&person));
            assert_eq!("Up", Wizard::fly(&person));
            assert_eq!("*waving arms furiously*", Human::fly(&person));

            // <Type as Trait>::function(reciever_if_method, next_arg, ...)
            assert_eq!(String::from("puppy"), <Dog as Animal>::baby_name());
            assert_eq!(String::from("Spot"), Dog::baby_name());

            let w = Wrapper(vec![String::from("hello"), String::from("world")]);
            // Vecの基本動作が使える
            for _ in w.0.iter() {
                // 何かの操作
            }
            // Displayトレイトが実装されているのでformatで出力できる
            assert_eq!("[hello, world]", format!("{}", w));
        }),
    );
}
