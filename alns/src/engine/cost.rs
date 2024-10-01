use std::arch::aarch64::vmax_f64;
use std::collections::HashMap;
use crate::coverage::coverage::Coverage;
use crate::input::input::InputData;

pub struct Score{
    optimization_score: f32,
    constraint_score: f32,
    coverage_score:f32,
    horizontal_coverage_score: f32,
    pattern_constraint_score: f32
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


    pub(crate) fn  calculate_horizontal_coverage_score(&self, input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

        0.0
    }

    pub(crate) fn  calculate_constraint_score(&self, input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

        0.0
    }

    pub(crate) fn  calculate_pattern_constraint_score(&self, input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

        0.0
    }

    pub(crate) fn  calculate_coverage_score(&self, input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

       0.0
    }


    pub(crate) fn calculate_total_score(&self, input_data: &InputData,  schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

        0.0
    }
}