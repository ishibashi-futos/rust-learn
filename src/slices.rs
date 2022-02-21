pub fn slices() {
    let s = String::from("hello world");
    println!("word will get the value 5: actual:{}", first_word(&s));

    println!(
        "word will get the value 11: actual:{}",
        first_word(&String::from("hello_world"))
    );

    let slice = &s[0..5];
    println!("slice: {}", slice);

    println!("expected: hello, actual: {}", mod_first_word(&s));
    println!(
        "expected: hello_world, actual: {}",
        mod_first_word(&String::from("hello_world"))
    );
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn mod_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
