pub fn match_flow_control() {
    let coin = Coin::Penny;
    println!("coin is: {}", value_in_cents(coin));
    let coin = Coin::Nickel;
    println!("coin is: {}", value_in_cents(coin));
    let coin = Coin::Dime;
    println!("coin is: {}", value_in_cents(coin));
    let coin = Coin::Quarter(UsState::Alaska);
    println!("coin is: {}", value_in_cents(coin));
    let coin = Coin::Quarter(UsState::Alabama);
    println!("coin is: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    write(&six);
    let five = Some(6);
    let seven = plus_one(five);
    write(&seven);
    let none: Option<i32> = None;
    let none = plus_one(none);
    write(&none);
    // plus_one(Some(255));

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(x) => println!("some value x is: {}", x),
        _ => (),
    }

    if let Some(x) = some_u8_value {
        println!("some value x is: {}", x);
    }
    let some_u8_value: Option<u8> = None;
    if let None = some_u8_value {
        println!("some value x is none");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alabama) => {
            println!("State quarter from {:?}!", UsState::Alabama);
            25
        }
        Coin::Quarter(UsState::Alaska) => {
            println!("State quarter from {:?}!", UsState::Alaska);
            30
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(255) => panic!("overflow"), // 特定の値の場合についても記述できる
        Some(i) => Some(i + 1),
    }
}

fn write(x: &Option<i32>) {
    match x {
        None => println!("value is None"),
        Some(i) => println!("value is {}", i),
    }
}
