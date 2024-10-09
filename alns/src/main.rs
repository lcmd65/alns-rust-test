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
mod violation;
mod executor;


#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;


#[cfg(not(target_env = "msvc"))]
#[global_allocator]

static GLOBAL: Jemalloc = Jemalloc;



fn main() {
    let input_data = json::read_input_data_from_file("src/resource/dump/data_dummy.json")
        .expect("Failed to read input data from JSON file");

    println!("[validate_input_data]");
    let mut alns = Alns::new(&input_data);
    println!("[alns start]");
    alns.run_iteration();
    println!("[end]");
}