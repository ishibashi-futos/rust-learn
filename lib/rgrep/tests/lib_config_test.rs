extern crate rgrep;

use rgrep::Config;

#[test]
fn new_argslength2_is_ok() {
    let args = vec![
        String::from("rgrep"),
        String::from("foo"),
        String::from("bar"),
    ];
    let iter = args.iter();

    let config = Config::new(iter).unwrap();
    assert_eq!(config.query, "foo");
    assert_eq!(config.filename, "bar");
}

#[test]
fn new_args_length_more_less_than_2_should_be_error() {
    let args = vec![String::from("rgrep"), String::from("foo")];
    let iter = args.iter();

    if let Err(str) = Config::new(iter) {
        assert_eq!(str, "Didn't get a filename string");
    } else {
        panic!("Should be error");
    }
}

#[test]
fn new_args_length_more_less_than_1_should_be_error() {
    let args = vec![String::from("rgrep")];
    let iter = args.iter();

    if let Err(str) = Config::new(iter) {
        assert_eq!(str, "Didn't get a query string");
    } else {
        panic!("Should be error");
    }
}
