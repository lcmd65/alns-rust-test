use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
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
}