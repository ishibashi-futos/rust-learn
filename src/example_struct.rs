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

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    square.print_area();
    println!("square is {}", square.is_square());
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

impl Rectangle {
    // Rectangle構造体にareaメソッドを定義する
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_area(&self) {
        println!(
            "The area of the rectangle is {} square pixels.",
            self.area()
        );
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// implブロックは分割することができる
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
