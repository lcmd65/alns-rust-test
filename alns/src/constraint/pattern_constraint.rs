use std::iter::Map;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Deserialize, Serialize)]
pub struct PatternConstraint {
    pub (crate) id: String,
    pub (crate) description: String,
    pub (crate) constraint_type: String,
    pub (crate) shift_patterns: Vec<String>,
    pub (crate) staff_groups: Vec<String>,
    pub (crate) priority: i8,
    pub (crate) exist: bool,
    pub (crate) is_hard: bool,
    pub (crate) penalty: i8,
    pub (crate) pattern_lists : Vec<HashMap<String, Vec<String>>>

}


impl PatternConstraint {}