extern crate rgrep;

use rgrep::search;

#[test]
fn search_one_result() {
    let query = "duct";

    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let actual = search(query, contents);

    assert_eq!(vec!["safe, fast, productive."], actual);
}

#[test]
fn search_match_two_lines_two_results() {
    let query = "Rust";

    let contents = "\
Rust:
safe, fast, productive.
Rust is awesome.
Pick three.";

    let actual = search(query, contents).len();

    assert_eq!(2, actual);
    assert_eq!(vec!["Rust:", "Rust is awesome."], search(query, contents));
}

#[test]
fn search_not_found_query_text() {
    let query = "Not Found";

    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let actual = search(query, contents).len();

    assert_eq!(0, actual);
}
