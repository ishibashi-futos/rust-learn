use rand::Rng;

pub fn flow_control() {
    let number = rand::thread_rng().gen_range(1, 10);

    if number < 5 {
        println!("condition was true: number={}", number);
    } else {
        println!("condition was false: number={}", number);
    }

    if number != 0 {
        println!("number was something other than zero: number={}", number);
    }

    let condition = rand::thread_rng().gen_range(1, 10) < 5;
    let number = if condition {
        "more than 5"
    } else {
        "less than 5"
    };
    println!("The number of: {}", number);

    let mut i = 0;
    loop {
        println!("loop: i={}", i);
        i += 1;
        if i > 10 {
            println!("loop break!");
            break;
        }
        println!("again!");
    }

    let mut i = 5;
    println!("Count down start!");
    while i != 0 {
        println!("while: i={}", i);
        i = i - 1;
    }
    println!("LIFT OFF!!");

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", arr[index]);
        index = index + 1;
    }

    // iteratorをつかってfor文を使う
    for element in arr.iter() {
        println!("the value is: {}", element);
    }
}
