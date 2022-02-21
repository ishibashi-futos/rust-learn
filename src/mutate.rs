pub fn mutate() {
    // let x = 5; // immutable
    let mut x: i32 = 5; // mutable
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // const MAX_LENGTH: usize = 100_000;
    const MAX_LENGTH: usize = 10;

    println!("Please enter message [max 10 characters]");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    if input.len() > MAX_LENGTH {
        panic!("Input is too long! length={}", input.len());
    } else {
        println!("Input is {} characters long.", input.len());
    }
}
