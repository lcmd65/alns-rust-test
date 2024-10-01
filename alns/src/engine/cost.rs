use std::arch::aarch64::vmax_f64;
use std::collections::HashMap;
use crate::coverage::coverage::Coverage;
use crate::input::input::InputData;

pub struct Score{
    optimization_score: f64,
    constraint_score: f64,
    coverage_score:f64,
    horizontal_coverage_score: f64,
    pattern_constraint_score: f64
}

impl Score {

    pub(crate) fn init() -> Self{

        Self {
            optimization_score : 0.0,
            constraint_score : 0.0,
            coverage_score : 0.0,
            horizontal_coverage_score : 0.0,
            pattern_constraint_score :0.0
        }
    }


    fn calculate_horizontal_coverage_score(input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f64{

        0.0
    }

    fn calculate_constraint_score(input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f64{

        0.0
    }

    fn calculate_pattern_constraint_score(input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f64{

        0.0
    }

    fn calculate_coverage_score(input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f64{

       0.0
    }


    fn calculate_total_score(input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f64{

        0.0
    }
}