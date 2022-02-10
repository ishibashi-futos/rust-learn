mod guess_the_number;
mod mutate;
mod data_types;
mod functions;
mod flow_control;

fn main() {
    println!("Hello, world!");
    guess_the_number::guess_the_number();
    mutate::mutate();
    data_types::data_types();
    functions::all();
    flow_control::flow_control();
    println!("Good bye!");
}
