use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct Constraint {
    pub (crate) id: String,
    pub (crate) score_formula: String,
    pub (crate) description: String,
    pub (crate) constraint_type: String,
    pub (crate) staff_groups: Vec<String>,
    pub (crate) priority: i8,
    pub (crate) to_maximize: bool,
    pub (crate) is_hard: bool,
}