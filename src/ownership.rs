pub fn ownership() {
    let s1 = String::from("hello");
    println!("s1: {}", &s1);
    let s2 = s1;
    println!("s2={}", &s2);

    // println!("s1={}", );
    println!("s2={}", &s2);

    let s3 = s2.clone();
    println!("s2: {}, s3: {}", &s2, &s3);

    let s4 = String::from("hello");
    tasks_ownership(s4);
    // println!("s4: {}", &s4); // can't use (moved)

    let x = 5;
    makes_copy(x);
    println!("x: {}", &x); // can use
}

fn tasks_ownership(some_string: String) {
    println!("some_string: {}", &some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", &some_integer);
}
