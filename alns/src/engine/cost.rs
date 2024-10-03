use std::collections::HashMap;
use crate::coverage::coverage::Coverage;
use crate::input::input::InputData;
use crate::violation::rule::Rule;
use crate::executor::formula_executor::FormulaExecutor;

pub struct Score<'a> {
    optimization_score: f32,
    constraint_score: f32,
    coverage_score:f32,
    horizontal_coverage_score: f32,
    pattern_constraint_score: f32,
    input: &'a InputData,
    rule: Rule<'a>,
    executor: FormulaExecutor
}

impl<'a> Score<'a> {

    pub(crate) fn new(input_data: &'a InputData) -> Self{

        let rule = Self {
            optimization_score : 0.0,
            constraint_score : 0.0,
            coverage_score : 0.0,
            horizontal_coverage_score : 0.0,
            pattern_constraint_score :0.0,
            rule: Rule::new(&input_data),
            input: input_data,
            executor: FormulaExecutor
        };

        rule
    }


    pub(crate) fn  calculate_horizontal_coverage_score(
        &self ,
        schedule: &HashMap<String,HashMap<i8, String>>
    ) -> f32 {
        let mut score = 0.0;

        for week in 1..= self.input.schedule_period {
            for horizontal_coverage in &self.input.horizontal_coverages {
                let number_violation = self.rule.calculate_number_horizontal_coverage_violation(&horizontal_coverage, &week, schedule);
                score += number_violation as f32 * horizontal_coverage.penalty as f32 * horizontal_coverage.priority as f32;
            }
        }

        -score
    }

    pub(crate) fn  calculate_coverage_score(
        &self,
        schedule: &HashMap<String, HashMap<i8, String>>
    ) -> f32{
        let mut score = 0.0;

        for week in 1..= self.input.schedule_period {
            for coverage in &self.input.coverages {
                let number_violation = self.rule.calculate_number_coverage_violation(&coverage, &week, schedule);
                score += number_violation as f32 * coverage.penalty as f32 * coverage.priority as f32;
            }
        }

        -score
    }

    pub(crate) fn  calculate_constraint_score(&self, schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

        let mut score = 0.0 ;
        for week in 1..=*&self.input.schedule_period{
            for constraint in &self.input.constraints{
                let mut map = self.rule.constraint_violation(&constraint, &week, &schedule);
                let score_map = self.executor.executor_rust(&constraint.score_formula, &mut map);
                for (_, value) in score_map {
                    score += value;
                }
            }
        }

        -score
    }

    pub(crate) fn  calculate_pattern_constraint_score(&self, schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

        -0.0
    }


    pub(crate) fn calculate_total_score(&self,  schedule: &HashMap<String,HashMap<i8, String>>) -> f32{

        self.calculate_horizontal_coverage_score(schedule) +
            self.calculate_coverage_score(schedule)+
            self.calculate_constraint_score( schedule) +
            self.calculate_pattern_constraint_score(schedule)
    }
}