pub fn example_struct() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect); // #で指定すると、構造体のプロパティ名を整形して表示する
}

// debugトレイトを使用する
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
