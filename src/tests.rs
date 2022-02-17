#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 7, height: 6 };

        let actual = larger.can_hold(&smaller);

        assert!(actual);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 7, height: 6 };

        let actual = smaller.can_hold(&larger);

        assert!(!actual);
    }

    #[test]
    fn add_two_4to6() {
        let actual = add_two(4);

        assert_eq!(actual, 6);
    }

    #[test]
    fn add_two_0to2() {
        let actual = add_two(0);

        assert_eq!(actual, 2);
    }

    #[test]

    fn greeting_contains_name() {
        let result = greeting("Carol");

        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 101.")]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 0.")]
    fn lower_than_0() {
        Guess::new(0);
    }

    #[test]
    fn guess_new_ok() {
        Guess::new(1);
    }

    #[test]
    fn it_work_is_ok() -> Result<(), String> {
        let result = it_work(2)?;

        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn it_work_is_err() {
        let result = it_work(4);

        assert_eq!(result, Err("two plus two does not equal four".to_string()));
    }
}

#[allow(dead_code)]
fn it_work(i: u32) -> Result<u32, String> {
    if i + 2 == 4 {
        Ok(i+2)
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(dead_code)]
fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

struct Guess {
    value: u8,
}

impl Guess {
    fn new(value: u8) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
