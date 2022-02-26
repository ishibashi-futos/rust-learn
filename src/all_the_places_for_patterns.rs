use crate::logger;

pub fn all_the_places_for_patterns() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        logger::info(&format!("Using your favorite color, {}", color));
    } else if is_tuesday {
        logger::info("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            logger::info("Using purple");
        } else {
            logger::info("Using orange");
        }
    } else {
        logger::info("Using blue");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(value) = stack.pop() {
        logger::info(&format!("{}", value));
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        logger::info(&format!("{} is at index {}", value, index));
    }

    let (x, y, z) = (1, 2, 3);
    logger::info(&format!("x={}, y={}, z={}", x, y, z));
    let (x, y, _) = (10, 20, 30); // タプルの値を無視するときは`_`を使う
    logger::info(&format!("x={}, y={}, z={}", x, y, z)); // x=10, y=20, z=3
    let (x, _, z) = (100, 200, 300);
    logger::info(&format!("x={}, y={}, z={}", x, y, z)); // x=100, y=20, z=300

    let point = (3, 5);
    print_cordinates(&point);
}

// タプルを`x`と`y`に分解する
fn print_cordinates(&(x, y): &(i32, i32)) {
    logger::info(&format!("Current location: ({}, {})", x, y));
}
