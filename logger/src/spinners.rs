pub mod spinners {
    pub struct Dots {
        chars: Vec<char>,
    }

    impl Dots {
        #[allow(dead_code)]
        pub fn new() -> Dots {
            Dots {
                chars: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            }
        }
    }

    impl Iterator for Dots {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            let item = self.chars[0];
            self.chars.rotate_left(1);
            Some(item)
        }
    }

    pub struct Moon {
        chars: Vec<char>,
    }

    impl Moon {
        #[allow(dead_code)]
        pub fn new() -> Moon {
            Moon {
                chars: vec!['🌑', '🌒', '🌓', '🌔', '🌕', '🌖', '🌗', '🌘'],
            }
        }
    }

    impl Iterator for Moon {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            let item = self.chars[0];
            self.chars.rotate_left(1);
            Some(item)
        }
    }

    pub struct Clock {
        chars: Vec<char>,
    }

    impl Clock {
        #[allow(dead_code)]
        pub fn new() -> Clock {
            Clock {
                chars: vec![
                    '🕐', '🕑', '🕒', '🕓', '🕔', '🕕', '🕖', '🕗', '🕘', '🕙', '🕚', '🕛',
                ],
            }
        }
    }

    impl Iterator for Clock {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            let item = self.chars[0];
            self.chars.rotate_left(1);
            Some(item)
        }
    }
}
