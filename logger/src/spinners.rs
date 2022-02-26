pub mod spinners {
    pub struct Dots {
        dots: Vec<char>,
    }

    impl Dots {
        #[allow(dead_code)]
        pub fn new() -> Dots {
            Dots {
                dots: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            }
        }
    }

    impl Iterator for Dots {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            self.dots.rotate_left(1);
            Some(self.dots[0])
        }
    }

    pub struct Moon {
        item: Vec<char>,
    }

    impl Moon {
        #[allow(dead_code)]
        pub fn new() -> Moon {
            Moon {
                item: vec!['🌑', '🌒', '🌓', '🌔', '🌕', '🌖', '🌗', '🌘'],
            }
        }
    }

    impl Iterator for Moon {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            self.item.rotate_left(1);
            Some(self.item[0])
        }
    }

    pub struct Clock {
        item: Vec<char>,
    }

    impl Clock {
        #[allow(dead_code)]
        pub fn new() -> Clock {
            Clock {
                item: vec![
                    '🕐', '🕑', '🕒', '🕓', '🕔', '🕕', '🕖', '🕗', '🕘', '🕙', '🕚', '🕛',
                ],
            }
        }
    }

    impl Iterator for Clock {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            self.item.rotate_left(1);
            Some(self.item[0])
        }
    }
}
