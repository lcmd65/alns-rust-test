use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct HorizontalCoverage {
    pub (crate) id: String,
    pub (crate) shifts: Vec<String>,
    pub (crate) staffs: Vec<String>,
    pub (crate) days: Vec<i8>,
    pub (crate) types: Vec<String>,
    pub (crate) desire_value: i8,
    pub (crate) penalty: i8,
    pub (crate) priority: i8,
}