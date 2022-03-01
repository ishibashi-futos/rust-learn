use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }

    if m > n {
        println!("No");
        return;
    }

    let mut m = BTreeMap::new();
    for i in a.iter() {
        let e = m.entry(*i).or_insert(0);
        *e += 1;
    }
    for l3 in b.iter() {
        let e = m.entry(*l3).or_insert(0);
        if *e > 0 {
            *e -= 1;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
    return;
}
