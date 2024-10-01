use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Constraint {
    pub (crate) id: String,
    pub (crate) score_formula: String,
    pub (crate) description: String,
    pub (crate) constraint_type: String,
    pub (crate) staff_groups: Vec<String>,
    pub (crate) priority: i8,
    pub (crate) to_maximize: bool,
    pub (crate) is_hard: bool,
    pub (crate) default_value: f32,
    pub (crate) threshold: f32,
    pub (crate) step: f32,
    pub (crate) score: f32,
    pub (crate) covert_kotlin_flag: bool,
    pub (crate) mid_search: bool
}