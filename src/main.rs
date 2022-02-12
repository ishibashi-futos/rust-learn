mod guess_the_number;
mod mutate;
mod data_types;
mod functions;
mod flow_control;
mod ownership;
mod references_and_borrowing;
mod slices;
mod use_struct;
mod example_struct;
mod defining_enum;
mod match_flow_control;

fn main() {
    println!("Hello, world!");
    guess_the_number::guess_the_number();
    mutate::mutate();
    data_types::data_types();
    functions::all();
    flow_control::flow_control();
    ownership::ownership();
    references_and_borrowing::references_and_borrowing();
    slices::slices();
    use_struct::use_struct();
    example_struct::example_struct();
    defining_enum::defining_enum();
    match_flow_control::match_flow_control();
    println!("Good bye!");
}
