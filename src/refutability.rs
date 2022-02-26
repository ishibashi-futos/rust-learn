use crate::logger;

pub fn refutability() {
    let some_option_value = Some("Hello");

    // Noneを受け取ることを想定していないコードであるため、コンパイルエラーになる
    // let Some(x) = some_option_value;

    // この場合、Noneである場合何もしないということが明確である
    if let Some(x) = some_option_value {
        logger::info(&format!("{}", x));
    }

    // パターンが１つしかあり得ないので、このコードはコンパイルエラーとなる
    // let x = 5 {
    //     logger::info(&format!("{}", x));
    // }

    for x in 0..10 {
        match x {
            0 => logger::info("zero"),
            1 => logger::info("one"),
            2 => logger::info("two"),
            3 => logger::info("three"),
            4 => logger::info("four"),
            5 => logger::info("five"),
            _ => logger::info("unknown"),
        }
    }

    let x = Some(50);
    #[allow(unused_variables)]
    let y = 10;

    match x {
        Some(50) => logger::info("Got 50"),
        Some(y) => logger::info(&format!("Matched, y = {:?}", y)), // 新しくyとして値が拘束されるので、Some(10)であるわけではない
        _ => logger::info(&format!("Default case, x = {:?}", x)),
    }

    let x = 1;

    match x {
        1 | 2 => logger::info("one or two"),
        3 => logger::info("three"),
        _ => logger::info("anything"),
    }

    let x = 5;

    match x {
        1..=5 => logger::info("one through five"),
        _ => logger::info("something else"),
    }

    let v = vec!['c', 'k', 'z', '1'];

    for x in v {
        match x {
            'a'..='j' => logger::info("early ASCII letter"),
            'k'..='z' => logger::info("late ASCII letter"),
            _ => logger::info("something else"),
        }
    }

    // 構造体のフィールドを変数に分解する
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    // 構造体のフィールドでmatchさせる

    let p = Point { x: 1, y: 0 };
    match_point(p); // Pattern1
    let p = Point { x: 0, y: 1 };
    match_point(p); // Pattern2
    let p = Point { x: 10, y: 4 };
    match_point(p); // Pattern3

    let m = Message::Quit;
    match_enum(m);

    let m = Message::Move { x: 10, y: 20 };
    match_enum(m);

    let m = Message::Write(String::from("Hoge"));
    match_enum(m);

    let m = Message::ChangeColor(255, 255, 255);
    match_enum(m);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    logger::info(&format!("{}-{}", feet, inches));
    logger::info(&format!("{}:{}", x, y));

    // 通常未使用の変数は警告が出るが、`_`を使うと警告が消える
    let _x = 5;
    let y = 10;
    logger::info(&format!("{}", y));
    // logger::info(&format!("{}", _x)); // 別に変数として使えないわけでもない

    let s = Some(String::from("Hello"));

    // `_`で定義すれば値がmoveされないので再度`s`を使うことができる
    if let Some(_) = s {
        logger::info("found a string");
    }

    logger::info(&format!("{:?}", s));

    // `..`で残りを無視したり、途中の値をすべて無視します
    let v = (1, 2, 3, 4, 5, 6);
    match v {
        (first, .., last) => logger::info(&format!("first={}, last={}", first, last)),
    }

    match v {
        (_, second, ..) => logger::info(&format!("second={}", second)),
    }

    match v {
        (_, _, third, .., last) => logger::info(&format!("third={}, last={}", third, last)),
    }

    // refキーワードを使用すると値がmoveされないので再使用できる
    let robot_name = Some(String::from("R2D2"));
    match robot_name {
        Some(ref name) => logger::info(&format!("Found a name: {}", name)),
        None => logger::info("No name."),
    }
    logger::info(&format!("robot name is: {:?}", robot_name));

    // 値を書き換えたいときは`ref mut`
    let mut robot_name = Some(String::from("R2D2"));
    match robot_name {
        Some(ref mut name) => *name = String::from("C3PO"),
        None => (),
    }
    logger::info(&format!("robot name is: {:?}", robot_name));

    // match guardを使って条件を追加する
    let num = Some(4);

    match num {
        Some(x) if x < 5 => logger::info("less than five"),
        Some(x) => logger::info(&format!("{}", x)),
        None => (),
    }

    // match guardを使って外部の変数で条件を追加する
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => logger::info("Got 50"),
        Some(n) if n == y => logger::info(&format!("Matched, n = {:?}", n)),
        _ => logger::info(&format!("Default case, x = {:?}", x)),
    }

    // or演算子`|`を使って複数パターンを指定する
    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => logger::info("4, 5, or 6"), // `(4 | 5 | 6) if y`のように評価される `4 | 5 | (6 if y)`ではない
        _ if y => logger::info("y is true"),          // 4でも5でも6でもないが、yがtrueならば
        _ => logger::info("anything"),                // どれでもない
    }

    // `@`演算子により、値を変数に拘束しつつパターンを満たすかどうかチェックできる
    let msg = GreetMessage::Hello { id: 5 };

    match msg {
        GreetMessage::Hello {
            id: id_variable @ 3..=7,
        } => {
            logger::info(&format!("Found an id in range: {}", id_variable));
        }
        GreetMessage::Hello { id: 10..=112 } => {
            logger::info("Found an id in another range");
        }
        GreetMessage::Hello { id } => {
            logger::info(&format!("Found some other id: {}", id));
        }
    }
}

enum GreetMessage {
    Hello { id: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

fn match_point(p: Point) {
    match p {
        Point { x, y: 0 } => logger::info(&format!("On the x axis at {}", x)), // Pattern1
        Point { x: 0, y } => logger::info(&format!("On the y axis at {}", y)), // Pattern2
        Point { x, y } => logger::info(&format!("On neither axis: ({}, {})", x, y)), // Pattern3
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn match_enum(m: Message) {
    match m {
        Message::Quit => logger::info("Quit"),
        Message::Move { x, y } => logger::info(&format!("Move to ({}, {})", x, y)),
        Message::Write(text) => logger::info(&format!("Write {}", text)),
        Message::ChangeColor(r, g, b) => {
            logger::info(&format!("Change the color to ({}, {}, {})", r, g, b))
        }
    }
}
