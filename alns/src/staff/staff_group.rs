use serde::{Deserialize, Serialize};
use crate::staff::staff::Staff;

#[derive(Deserialize, Serialize, PartialEq)]
pub struct StaffGroup{
    pub (crate) id : String,
    pub (crate) staff_list: Vec<String>
}