use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line1 = input.trim().parse::<i32>().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line2: i32 = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .sum();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line3 = input.trim();
    println!("{} {}", line1 + line2, line3);
}
