use crate::constraint::constraint::Constraint;
use crate::constraint::pattern_constraint::PatternConstraint;
use crate::coverage::horizontal_coverage::HorizontalCoverage;

pub mod constraint;
pub mod pattern_constraint;


pub enum InterfaceConstraint {
    Constraint(Constraint),
    PatternConstraint(PatternConstraint),
    HorizontalCoverage(HorizontalCoverage)
}