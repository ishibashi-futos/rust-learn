use std::fmt::Display;

use crate::logger;

pub fn lifetime_syntax() {
    let r;
    {
        let x = 5;
        // r = &x; // コンパイルエラーになる
        r = x; // 値をCopyしているのでコンパイルエラーにならない
    }
    logger::info(&format!("r: {}", r));

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    logger::info(&format!("result: {}", result));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    logger::info(&format!("{}", first_sentence));
    logger::info(&format!("{}", i.part));
    display_important_excerpt(i);

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let level = i.level();
    // drop(i); // dropでiを破棄しても level は破棄されない
    logger::info(&format!("level: {}", level));

    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let part = i.announce_and_return_part("Hello, world!");
    // drop(i); // dropすると、partは使えなくなる
    logger::info(&format!("part: {}", part));

    let s1 = String::from("hello world");
    let s2 = String::from("good bye");
    let result = longest_with_an_announcement(&s1, &s2, "longest");
    logger::info(&format!("result: {}", result));
}

// &strでは、返している参照の値がa, bのどちらかになるかコンパイラは理解できない
// &'a str は、明示的なライフタイム付きの参照であることを表す
// &'a mut str は明示的なライフタイム付きの可変参照であることを表す
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ImportantExcerptのインスタンスが作られる前にデータは保持されますが
// ライフタイム注釈をつけると、ImportantExcerptのインスタンスがスコープを抜けるまで参照が有効になる
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn display_important_excerpt(i: ImportantExcerpt) {
    logger::info(&format!("{:?}", i));
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        logger::info(&format!("Attention please: {}", announcement));
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    logger::info(&format!("Announcement: {}", ann));
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
