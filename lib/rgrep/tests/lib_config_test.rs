extern crate rgrep;

use rgrep::Config;

#[test]
fn new_argslength2_is_ok() {
    let args = vec![
        String::from("rgrep"),
        String::from("foo"),
        String::from("bar"),
    ];

    let config = Config::new(&args).unwrap();
    assert_eq!(config.query, "foo");
    assert_eq!(config.filename, "bar");
}

#[test]
fn new_args_length_less_than_2_should_be_error() {
    let args = vec![String::from("rgrep"), String::from("foo")];

    if let Err(str) = Config::new(&args) {
        assert_eq!(str, "not enough arguments");
    } else {
        panic!("Should be error");
    }
}
