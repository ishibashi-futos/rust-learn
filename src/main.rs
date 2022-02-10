mod guess_the_number;
mod mutate;
mod data_types;

fn main() {
    println!("Hello, world!");
    guess_the_number::guess_the_number();
    mutate::mutate();
    data_types::data_types();
    println!("Good bye!");
}
