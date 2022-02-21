pub fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_4_count_called_is_none() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        v1_iter.next();
        v1_iter.next();
        v1_iter.next();
        let none = v1_iter.next();

        assert_eq!(none, None);
    }

    #[test]
    fn iterator_sum_called_skip_1() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        // 1回スキップすることもできる
        v1_iter.next();
        let actual_total: i32 = v1_iter.sum();

        assert_eq!(actual_total, 5);
    }

    #[test]
    fn map_add_all_items_plus_one() {
        let v1 = vec![1, 2, 3];

        let actual_values: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(actual_values, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_my_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let actual_in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            actual_in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        )
    }

    #[test]
    fn counter_generate_less_than_6_values() {
        let counter = Counter::new();

        for i in counter.into_iter() {
            assert!(i < 6);
        }
    }

    #[test]
    fn counter_called_6_counts_return_none_value() {
        let mut counter = Counter::new();

        counter.next();
        counter.next();
        counter.next();
        counter.next();
        counter.next();

        let actual_value_is_none = counter.next();
        assert_eq!(actual_value_is_none, None);
    }

    #[test]
    fn can_using_iterator_trait_methods() {
        let counter = Counter::new();

        let sum: u32 = counter
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    #[allow(dead_code)]
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
