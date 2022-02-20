extern crate rust_learn;

use rust_learn::closures::*;

#[test]
fn value_cached_value_found() {
    let mut cache = MappedCache::new(|v| {
        v * v
    });

    let v1 = cache.value(2);

    assert_eq!(v1, 4);
}

#[test]
fn value_cached_two_value_found() {
    let mut cache = MappedCache::new(|v| {
        v * v
    });
    cache.value(2);

    let v1 = cache.value(4);

    assert_eq!(v1, 16);
}
