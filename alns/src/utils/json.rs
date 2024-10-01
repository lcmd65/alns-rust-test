use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use crate::input::input::InputData;

pub(crate) fn read_input_data_from_file(file_path: &str) -> std::io::Result<InputData> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize JSON string to InputData struct
    let input_data: InputData = serde_json::from_str(&contents)?;
    Ok(input_data)
}