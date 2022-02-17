use std::fmt;
use std::fmt::Display;

use crate::logger;

pub fn generics_example() {
    let number_list: Vec<u32> = vec![34, 50, 25, 100, 65];
    let largest_item = largest_u32(&number_list);
    logger::info(&format!(
        "The largest number is {}. in: {:?}",
        largest_item, number_list
    ));

    let number_list: Vec<i32> = vec![34, -50, 25, -100, 65];
    let largest_item = largest_i32(&number_list);
    logger::info(&format!(
        "The largest number is {}. in: {:?}",
        largest_item, number_list
    ));

    let item_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let largest_item = largest_char(&item_list);
    logger::info(&format!(
        "The largest char is {}. in: {:?}",
        largest_item, item_list
    ));

    let number_list: Vec<u32> = vec![34, 50, 25, 100, 65];
    let largest_item = largest(&number_list);
    logger::info(&format!(
        "The largest number is {}. in: {:?}",
        largest_item, number_list
    ));

    let number_list: Vec<i32> = vec![34, -50, 25, -100, 65];
    let largest_item = largest(&number_list);
    logger::info(&format!(
        "The largest number is {}. in: {:?}",
        largest_item, number_list
    ));

    let item_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let largest_item = largest(&item_list);
    logger::info(&format!(
        "The largest char is {}. in: {:?}",
        largest_item, item_list
    ));

    let new_post = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // logger::info(&format!("new post: {}", new_post.summarize()));
    notify(&new_post);
    notify_and_display(&new_post);

    let new_post = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best\nhockey team in the NHL.",
        ),
    };
    // logger::info(&format!("new post: {}", new_post.summarize()));
    notify(&new_post);
    notify_and_display(&new_post);

    let new_post = Youtube {
        username: String::from("La+Darknesss"),
        title: String::from("10 Steps to becoming a better developer"),
        description: String::from("of course, as you probably already know, people"),
    };
    // logger::info(&format!("new post: {}", new_post.summarize()));
    notify(&new_post);
    // notify_and_display(&new_post); // Youtube構造体はDisplayを実装していないのでエラーになる

    let pair = Pair { x: 5, y: 10 };
    pair.cmp_display();
    logger::info(&format!("pair: {}", pair.to_string()));

    let pair = Pair { x: 10, y: 5 };
    pair.cmp_display();
    logger::info(&format!("pair: {}", pair.to_string()));
}

fn largest_u32(numbers: &[u32]) -> u32 {
    let mut max = &numbers[0];
    for number in numbers {
        if number > max {
            max = number;
        }
    }
    *max
}

fn largest_i32(numbers: &[i32]) -> i32 {
    let mut max = &numbers[0];
    for number in numbers {
        if number > max {
            max = number;
        }
    }
    *max
}

fn largest_char(list: &[char]) -> char {
    let mut max = &list[0];
    for number in list {
        if number > max {
            max = number;
        }
    }
    *max
}

// i32やcharはスタックに置けるのでCopyトレイトを実装している
// ジェネリクス型にしたことによって、データがスタックに置けるかわからなくなってしまったので、
// `Copy` Trait の実装を強制することによって、データがスタックに置けるようになる
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &number in list {
        if number > max {
            max = number;
        }
    }
    max
}

pub trait Summary {
    fn author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, by {} ({})",
            self.headline, self.author, self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

pub struct Youtube {
    pub username: String,
    pub title: String,
    pub description: String,
}

impl Summary for Youtube {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }

    // summarizeメソッドはデフォルト実装があるので実装しなくても怒られない
}

// `<T: Summary>(item: &T)`の糖衣構文(Symtax sugar)
pub fn notify(item: &impl Summary) {
    logger::info(&format!("New Post!: {}", item.summarize()));
}

// `Display` Traitは`std::fmt::Display`のimportが必要
pub fn notify_and_display(item: &(impl Summary + Display)) {
    logger::info(&format!("New Post!: {}", item.summarize()));
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            logger::info(&format!("The largest member is x = {}", self.x));
        } else {
            logger::info(&format!("The largest member is y = {}", self.y));
        }
    }
}

// Displayトレイトが実装されている任意の型であれば、
// 表示に使うことができることがわかるのでformat!マクロで使用できる
impl<T: Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}
