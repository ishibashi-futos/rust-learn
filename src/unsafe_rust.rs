use crate::logger;
use std::slice;

pub static HELLO_WORLD: &str = "Hello, world!";

pub fn unsafe_rust() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    logger::info(&format!("r1 is: {:?}", r1));
    logger::info(&format!("r2 is: {:?}", r2));

    let address = 0x012345usize;
    let r = address as *mut i32;
    logger::info(&format!("r is: {:?}", r));

    unsafe {
        logger::info(&format!("*r1 is: {}", *r1));
        logger::info(&format!("*r2 is: {}", *r2));
    }

    // dangerous(); // unsafeな関数はunsafeブロックで囲む必要がある
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (l, r) = split_at_mut(r, 3);
    logger::info(&format!("l: {:?}", l));
    logger::info(&format!("r: {:?}", r));

    let address = 0x012345usize;
    #[allow(unused_variables)]
    let r = address as *mut i32;

    // メモリを確保していないのにこの操作をしようとするのでsegmentation faultするだろう
    // let slice = unsafe {
    //     slice::from_raw_parts_mut(r, 10000)
    // };
    // logger::info(&format!("slice: {:?}", slice));

    unsafe {
        logger::info(&format!("Absolute value of -3 according to C: {}", abs(-3)));
    }

    call_from_c();

    logger::info(HELLO_WORLD);

    let mut handlers = vec![];
    for _ in 0..100 {
        let handler = std::thread::spawn(|| {
            add_to_count();
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }

    // 可変なstatic変数の読み書きはunsafeになる
    unsafe {
        logger::info(&format!("count: {}", COUNTER));
    }
}

unsafe fn dangerous() {
    // unsafe関数の中はunsafeなブロックになる
    logger::info("Unsafed");
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    // sliceを２回借用することになるので、`split_at_mut`はunsafeでないと使えない
    // (&mut slice[..mid], &mut slice[mid..])

    let ptr = slice.as_mut_ptr();
    // unsafeを関数のブロックに閉じ込める
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

pub static mut COUNTER: u32 = 0;
pub fn add_to_count() {
    unsafe {
        COUNTER += 1;
    }
}
