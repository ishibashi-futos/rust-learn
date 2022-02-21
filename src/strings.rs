use crate::logger;

pub fn strings() {
    let mut s = String::new();
    s.push_str("hello");

    // let mut s = "initial contents";
    // s.push_str(", world"); // String は、文字列を追加できない

    logger::info(&format!("s: {}", s));

    let mut s = String::from("lo");
    s.push('l');
    logger::info(&format!("s: {}", s));

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = s1 + &s2; // s1はmoveされ、もう使用できない
    logger::info(&format!("s3: {}", s3));

    // let s1 = String::from("tic");
    // let first = s1[0]; // 添字でのアクセスは許可されていないのでコンパイルエラー

    // let hello = "Здравствуйте";
    // logger::info(&format("first: {}", &hello[0])); // アスキーコードは文字列ではないためコンパイルエラー

    let chars = ['न', 'म', 'स', '्', 'त', 'े'];
    for c in chars.iter() {
        logger::info(&format!("c: {}", c));
    }

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // 最初の4byteを取得
    logger::info(&format!("s: {}", s)); // Зд
    let s = &hello[0..2]; // 最初の2byteを取得
    logger::info(&format!("s: {}", s)); // З
                                        // let s = &hello[0..1]; // 最初の1byteを取得しpanicする
                                        // logger::info(&format!("s: {}", s)); // Зд
}
