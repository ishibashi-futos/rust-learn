use crate::logger;

pub fn example_collections() {
    logger::info("example_collections");

    let v: Vec<i32> = Vec::new();
    logger::info(&format!("v: {:?}", v));
    let v = vec![1, 2, 3];
    logger::info(&format!("v: {:?}", v));

    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    logger::info(&format!("v: {:?}", v));

    let mut v: Vec<String> = Vec::new();
    let mut one = String::from("1");
    v.push(one); // 所有権が移るので、ここでoneは破棄される
                 // logger::info(&format!("one: {}", one)); // コンパイルできない
    logger::info(&format!("v: {:?}", v));
    one = String::from("2");
    logger::info(&format!("one: {}", one)); // 所有権は移ってない？
    logger::info(&format!("v: {:?}", v)); // 値がコピーされているので1が表示される

    // ベクタの要素を取り出す
    let v: Vec<u8> = vec![0, 1, 2, 3];
    let third: &u8 = &v[2];
    logger::info(&format!("third: {}", third));
    let third = v.get(2);
    logger::info(&format!("third: {}", third.unwrap()));

    // 以下の行はPanicを起こす
    // let does_not_exist = &v[100];
    // logger::info(&format!("does_not_exist: {}", does_not_exist));

    // 以下の行はPanicを起こさない
    let does_not_exist = v.get(100);
    logger::info(&format!(
        "does_not_exist: {}",
        does_not_exist.unwrap_or(&255)
    ));

    // ベクタ要素を取り出して使用しようとしたとき、その間で要素を追加するとコンパイルできなくなる
    let mut v = vec![100, 32, 57];
    v.push(22); // 参照される前なら追加できる
    let first = &v[0];
    // v.push(22); // 要素追加の過程でメモリアドレスが変化してしまう可能性があるため、コンパイルが許可されない
    logger::info(&format!("first: {}", first));

    // ベクタの各要素を走査する
    let v = vec![100, 32, 57];
    for i in &v {
        logger::info(&format!("i: {}", i));
    }

    for (i, &value) in v.iter().enumerate() {
        logger::info(&format!("i: {}, v: {}", i, value));
    }

    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for cell in &row {
        logger::info(&format!("cell: {:?}", cell));
    }

    // row.iter().enumerate() は Copy Traitを満たしていないためコンパイルできないと言われた
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
