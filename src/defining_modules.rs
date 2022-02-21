pub fn defining_modules() {
    // 絶対パスでモジュールを参照する
    crate::defining_modules::front_of_house::hosting::add_to_waitlist();
    crate::defining_modules::front_of_house::hosting::seat_at_table();

    crate::defining_modules::front_of_house::serving::take_order();
    crate::defining_modules::front_of_house::serving::serve_order();
    crate::defining_modules::front_of_house::serving::take_payment();

    // 相対パスでモジュールを参照する
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();

    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();

    // superキーワード
    back_of_house::fix_incorrect_order();

    // 構造体を公開する
    eat_at_restaurant();

    let appetizer = back_of_house::Appetizer::Soup;
    appetizer.print();
    let appetizer = back_of_house::Appetizer::Salad;
    appetizer.print();
    let appetizer = back_of_house::Appetizer::Bread;
    appetizer.print();
    let appetizer = back_of_house::Appetizer::Marinade;
    appetizer.print();
}

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
        pub fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("take_order");
        }

        pub fn serve_order() {
            println!("serve_order");
        }

        pub fn take_payment() {
            println!("take_payment");
        }
    }
}

fn serve_order() {
    front_of_house::serving::serve_order();
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        println!("fix_incorrect_order");
        cook_order();
        super::serve_order();
    }

    fn cook_order() {
        println!("cook_order");
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn print(&self) {
            println!(
                "toast: {}, seasonal fruit: {}",
                self.toast, self.seasonal_fruit
            );
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
        Bread,
        Marinade,
    }

    impl Appetizer {
        pub fn print(&self) {
            println!("Appetizer is: {:?}", self);
        }
    }
}

fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("Pears"); // seasonal_fruitはprivateなのでアクセスできない

    meal.print();
}
