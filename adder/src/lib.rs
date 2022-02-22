pub fn add_three(x: i32) -> i32 {
    x + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(8, add_three(4));
    }
}
