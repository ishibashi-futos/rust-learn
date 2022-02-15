use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write, Read};
use crate::logger;

pub fn errors_with_result() {
    logger::info("errors_with_result begin");

    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(new_file) => new_file,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    f.write_all("hello".as_bytes()).expect("Couldn't write to file");
    f.try_clone().expect("Failed to close file");

    // `unwrap`は`Result`を返す関数を呼び出したときに、`Result<T,E>`の`T`を取得できる
    // `Ok<T>`ではなかった場合はpanicする
    let s = read_text_from_file().unwrap();
    logger::info(&format!("hello.txt(by read_text_from_file): {}", s));
    let mut s = String::new();
    // `expect`は`unwrap`と動きは似ているが、エラーメッセージを渡すことができる
    read_text_from_file2(&mut s).expect("Couldn't read from file");
    logger::info(&format!("hello.txt(by read_text_from_file2): {}", s));
    let s = read_text_from_file3().unwrap();
    logger::info(&format!("hello.txt(by read_text_from_file3): {}", s));

    let s = read_text_from_file4().unwrap();
    logger::info(&format!("hello.txt(by read_text_from_file4): {}", s));

    fs::remove_file("hello.txt").expect("Failed to remove file");

    logger::info("errors_with_result end");
}

fn read_text_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_text_from_file2(s: &mut String) -> Result<(), io::Error> {
    let mut f = File::open("hello.txt")?;
    match f.read_to_string(s) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

fn read_text_from_file3() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // `?`をつけることでエラーがあった場合Err<T>を返してくれる
    // `?`で返すようにすると、`From Trait`でfrom関数を通すことで、
    // エラー型が戻り値に指定した`io::Error`に自動的に変換してくれる
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// `?`演算子をつけることで超シンプルに書ける
fn read_text_from_file4() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}