use std::fmt::{Display, self};

use crate::logger;

pub fn panic_or_not_panic() {
    let guess = Guess::new(10);

    let result = guess.value();

    logger::info(&format!("The result is: {}", result));

    let guess = Guess::create(99).unwrap();
    let result = guess.value();
    logger::info(&format!("The result is: {}", result));
    let guess = Guess::create(1).unwrap();
    let result = guess.value();
    logger::info(&format!("The result is: {}", result));

    let guess = Guess::create(100);
    match guess {
        Ok(guess) => {
            let result = guess.value();
            logger::info(&format!("The result is: {}", result));
        },
        Err(error) => {
            logger::info(&format!("Error: {}", error));
        }
    }

    let guess = Guess::create(101);
    match guess {
        Ok(guess) => {
            let result = guess.value();
            logger::info(&format!("The result is: {}", result));
        },
        Err(error) => {
            logger::info(&format!("Error: {}", error));
        }
    }

    let guess = Guess::create(0);
    match guess {
        Ok(guess) => {
            let result = guess.value();
            logger::info(&format!("The result is: {}", result));
        },
        Err(error) => {
            logger::info(&format!("Error: {}", error));
        }
    }

    // unwrap_orでデフォルト値を返すことができる
    let guess = Guess::create(255).unwrap_or(Guess::new(99)).value();
    logger::info(&format!("The result is: {}", guess));
}

pub struct Guess {
    value: u8,
}

impl Guess {
    pub fn new(value: u8) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value); // panicはコードが回復不可能であることを表す
        }

        Guess {
            value
        }
    }

    pub fn create(value: u8) -> Result<Guess, OutOfRange> {
        if value > 100 {
            return Err(OutOfRange::Higher(value));
        } else if value < 1 {
            return Err(OutOfRange::Lower(value));
        }

        Ok(Guess {
            value
        })
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Debug)]
pub enum OutOfRange {
    Lower(u8),
    Higher(u8),
}

impl Display for OutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OutOfRange::*;

        match self {
            Lower(value) => write!(f, "The value must be greater than or equal to 1. actual: {}", value),
            Higher(value) => write!(f, "The value must be less than or equal to 100. actual: {}", value),
        }
    }
}