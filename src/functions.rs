pub fn all() {
    another_function(5, 6);
    formula();
    println!("The value of: {}", five());
    let (five, six) = five_and_six();
    println!("The value five of: {}", five);
    println!("The value six of: {}", six);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", &x);
    println!("The value of y is: {}", &y);
}

fn formula() {
    let y = {
        let x = 3;
        x + 1 // セミコロンをつけると、ここで定義された変数は使えない
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    return 5;
}

fn five_and_six() -> (i32, i32) {
    (5, 6)
}