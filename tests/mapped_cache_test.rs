extern crate rust_learn;

use rust_learn::closures::*;

#[test]
fn value_cached_value_found() {
    let mut cache = MappedCache::new(|v| v * v);

    let v1 = cache.value(2);

    assert_eq!(v1, 4);
}

#[test]
fn value_cached_two_value_found() {
    let mut cache = MappedCache::new(|v| v * v);
    cache.value(2);

    let v1 = cache.value(4);

    assert_eq!(v1, 16);
}

#[test]
fn value_string_slice_cache_found() {
    let mut cache = MappedCache::new(|v: &str| v);

    cache.value("hello");
    let cached = cache.value("hello");

    assert_eq!(cached, "hello");
}

#[test]
fn value_string_slice_has_two_values_cache_found() {
    let mut cache = MappedCache::new(|v: &str| v);

    cache.value("hello");
    cache.value("world");
    let cached = cache.value("hello");

    assert_eq!(cached, "hello");
}

#[test]
fn value_is_none_generate_default_value() {
    let mut cache = MappedCache::new(|v: &str| v);

    let cached =
        cache.value_or_default("hello", |arg| if arg.len() > 10 { "yay!" } else { "nay!" });

    assert_eq!(cached, "nay!");
}

#[test]
fn value_is_found_value_hello() {
    let mut cache = MappedCache::new(|v: &str| v);

    cache.value("hello");
    let cached = cache.value_or_default("hello", |_| "world");

    assert_eq!(cached, "hello");
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct TestValue {
    item: u32,
}

#[test]
fn test_generator_func() {
    let mut cache = MappedCache::new(|v: &TestValue| v);

    cache.value(&TestValue { item: 1 });
    let cached = cache.value_or_default(&TestValue { item: 2 }, |_| &TestValue { item: 0 });

    assert_eq!(cached.item, 0);
}
