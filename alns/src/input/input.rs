use rand::prelude::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::constraint::constraint::Constraint;
use crate::constraint::pattern_constraint::PatternConstraint;
use crate::coverage::coverage::Coverage;
use crate::coverage::horizontal_coverage::HorizontalCoverage;
use crate::staff::staff::Staff;
use crate::shift::shift::Shift;
use crate::shift::shift_group::ShiftGroup;
use crate::staff::staff_group::StaffGroup;


#[derive(Deserialize, Serialize)]
pub struct InputData{
    pub (crate) schedule_period: i8,
    pub (crate) staffs: Vec<Staff>,
    pub (crate) staff_groups: Vec<StaffGroup>,
    pub (crate) coverages: Vec<Coverage>,
    pub (crate) horizontal_coverages: Vec<HorizontalCoverage>,
    pub (crate) constraints: Vec<Constraint>,
    pub (crate) pattern_constraints: Vec<PatternConstraint>,
    pub (crate) shifts: Vec<Shift>,
    pub (crate) shift_groups: Vec<ShiftGroup>,
    pub (crate) start_date: Date,
    pub (crate) public_holidays: Vec<Date>
}

#[derive(Deserialize, Serialize)]
pub struct Date{
    pub (crate) day: i8,
    pub (crate) month: i8,
    pub (crate) year: i16
}
impl  InputData {
    fn init(){


    }
}