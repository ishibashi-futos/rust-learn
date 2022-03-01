use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let second = nums.first().unwrap();
    let third = nums.get(*second as usize).unwrap();
    let fourth = nums.get(*third as usize).unwrap();
    println!("{}", fourth);
}
