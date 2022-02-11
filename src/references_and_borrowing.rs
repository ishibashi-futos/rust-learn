pub fn references_and_borrowing() {
    let s1 = String::from("hello");
    let len = caluculate_length(&s1); // 参照を渡す

    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");
    // 可変な参照を生成する &mut xxx
    change(&mut s1);
    println!("s1: {}", &s1);

    // 特定のスコープであるデータに対して可変の参照は1つしか持てないので以下の4行はビルドできない
    // let mut s1 = String::from("Hello, World!");
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("r1: {}, r2: {}", &r1, &r2);

    // imutableな参照を2つ生成できる
    let s1 = String::from("Hello, World!");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", &r1, &r2);


    // スコープが異なる場合は参照を生成できる
    #[allow(unused_variables, unused_mut)]
    let mut s1 = String::from("Hello, World!");
    {
        let r1 = &s1;
        println!("r1: {}", &r1);
    }
    {
        let r2 = &s1;
        println!("r2: {}", &r2);
    }

    #[allow(unused_variables, unused_mut)]
    let mut s1 = String::from("Hello, World!");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", &r1, &r2);
    // imutableな参照が残っている場合は、mutableな参照を生成できないので以下はビルドできない
    // let r3 = &mut s1;
    // println!("r1: {}, r2: {}, r3: {}", &r1, &r2, &r3);
    println!("no_dangle: {}", no_dangle());
}

// 参照で受け取る -> 所有権を受け取ることなく値を参照できる
fn caluculate_length(s: &String) -> usize {
    s.len()
}

// 可変な参照を受け取る -> 参照を受け取って、その参照を変更することができる
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ダングリングポインタを生成してしまわないよう、以下の関数はコンパイルエラーになる
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // ここで変数sはスコープをぬけ、ドロップされ、そのメモリは消されてしまう
// ポインタがさすメモリアドレスには、異なる値が既に格納されているかもしれないし、何もないかもしれない

// Stringを直接返すことができる
fn no_dangle() -> String {
    return String::from("No dangle string");
}