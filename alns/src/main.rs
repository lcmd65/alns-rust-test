use crate::engine::alns::Alns;
use crate::utils::json;

mod engine;
mod input;
mod staff;
mod coverage;
mod shift;
mod solution;
mod utils;
mod constraint;


fn main() {
    let input_data = json::read_input_data_from_file("src/resource/dump/data_dummy.json")
        .expect("Failed to read input data from JSON file");

    println!("[validate_input_data]");
    let mut alns = Alns::init(input_data);
    alns.run_iteration();
    println!("[end]");
}