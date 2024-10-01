use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ShiftGroup{
    pub (crate) id: String,
    pub (crate) shifts: Vec<String>
}