extern crate logger;

#[test]
fn shift_once_dot() {
    let mut dots = logger::spinners::spinners::Dots::new();

    let actual = dots.next().unwrap();

    assert_eq!(actual, '⠋');
}

#[test]
fn shift_twiced_dot() {
    let mut dots = logger::spinners::spinners::Dots::new();

    dots.next();
    let actual = dots.next().unwrap();

    assert_eq!(actual, '⠙');
}

#[test]
fn shift_10th_dot() {
    let mut dots = logger::spinners::spinners::Dots::new();

    dots.next();
    dots.next();
    dots.next();
    dots.next();
    dots.next();
    dots.next();
    dots.next();
    dots.next();
    dots.next();
    dots.next();
    let actual = dots.next().unwrap();
    assert_eq!(actual, '⠋');
}
