use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

extern crate hello_macro;

type Kilimeters = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn make_do_twice() -> Box<dyn Fn(fn(i32) -> i32, i32) -> i32> {
    Box::new(do_twice)
}

#[derive(HelloMacro)]
struct Test {}

pub fn advanced_types() {
    let x: i32 = 5;
    let y: Kilimeters = 5;

    logger::logger::info(&format!("x + y = {}", x + y));

    let f: Thunk = Box::new(|| {
        logger::logger::info("This is a thunk!");
    });

    f();

    let answer = do_twice(add_one, 5);
    assert_eq!(answer, 12);

    let list_of_numbers = vec![1, 2, 3];

    let list_of_strings = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    for v in list_of_strings {
        logger::logger::info(&format!("{}", v));
    }

    let dt = make_do_twice();

    let answer = dt(add_one, 5);
    assert_eq!(answer, 12);

    Test::hello_macro();
}
