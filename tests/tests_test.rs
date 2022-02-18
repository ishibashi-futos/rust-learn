extern crate rust_learn;
use rust_learn::tests::{tests_learn::*, it_work};
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_learn::add_two(2));
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
