pub fn ownership() {
    let s1 = String::from("hello");
    println!("s1: {}", &s1);
    let s2 = s1;
    println!("s2={}", &s2);

    // println!("s1={}", );
    println!("s2={}", &s2);

    let s3 = s2.clone();
    println!("s2: {}, s3: {}", &s2, &s3);

    let s4 = String::from("hello");
    tasks_ownership(s4);
    // println!("s4: {}", &s4); // can't use (moved)

    let x = 5;
    makes_copy(x);
    println!("x: {}", &x); // can use
}

fn tasks_ownership(some_string: String) {
    println!("some_string: {}", &some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", &some_integer);
}

#[derive(PartialEq, Debug)]
struct MyValue {
    item: u32,
}

#[allow(dead_code)]
impl MyValue {
    fn new(item: u32) -> MyValue {
        MyValue { item }
    }

    fn add(&self, b: u32) -> MyValue {
        MyValue {
            item: self.item + b,
        }
    }
}

#[allow(dead_code)]
fn append_values(values: &mut Vec<MyValue>) {
    let mut a = MyValue::new(100);
    {
        a.item = 130;
        let a = a.add(32);
        values.push(a);
    }
    values.push(a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values() {
        let mut values: Vec<MyValue> = Vec::new();

        append_values(&mut values);

        assert_eq!(values, vec![MyValue::new(162), MyValue::new(130),]);
    }
}
