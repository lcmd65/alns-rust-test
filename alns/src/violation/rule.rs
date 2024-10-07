use std::cmp::max;
use std::collections::HashMap;
use crate::constraint::constraint::Constraint;
use crate::constraint::InterfaceConstraint;
use crate::constraint::pattern_constraint::PatternConstraint;
use crate::input::input::InputData;
use crate::coverage::coverage::Coverage;
use crate::coverage::horizontal_coverage::HorizontalCoverage;
use crate::solution::solution;
use crate::utils::date;

pub struct Rule<'a> {
    input: &'a InputData
}

impl<'a> Rule<'a> {

    pub fn new(input_data: &'a InputData) -> Rule {
        Self{
            input: input_data
        }
    }


    pub fn calculate_number_coverage_fulfill(
        &self,
        coverage: &Coverage,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    ) -> i8 {
        let mut num_violation = 0;

        for staff_group_id in &coverage.staff_groups {
            let staff_group = &self.input.staff_groups.iter().find(|&x| x.id == *staff_group_id).unwrap();
            for staff in &staff_group.staff_list{
                if coverage.shift.contains(&schedule[*&staff].get(&(&coverage.day -1 + 7 * (week - 1))).unwrap()) {
                    num_violation += 1;
                }
            }
        }

        num_violation
    }

    pub fn calculate_number_coverage_violation(
        &self,
        coverage: &Coverage,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    ) -> i8{
        let coverage_fulfill = self.calculate_number_coverage_fulfill(&coverage, &week, schedule);

        if coverage.types.contains(&"at least".to_string()) {
            return max(&coverage.desire_value - coverage_fulfill, 0)
        }
        else if coverage.types.contains(&"equal to".to_string()){
            return max(&coverage.desire_value - coverage_fulfill, coverage_fulfill -coverage.desire_value)
        }
        else if coverage.types.contains(&"at most".to_string()){
            return max(coverage_fulfill - &coverage.desire_value, 0)
        }

        -1
    }

    pub fn calculate_number_horizontal_coverage_fulfill(
        &self,
        coverage: &HorizontalCoverage,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    ) -> HashMap<String, i8>{
        let mut num_coverage : HashMap<String, i8> = HashMap::new();

        if coverage.staffs.contains(&"all_staffs".to_string()){
            for staff in &self.input.staffs{
                let mut num_staff = 0;
                for day in &coverage.days{
                    if coverage.shifts.contains(schedule[&staff.id].get(&(day + 7 * (week - 1))).unwrap()) {
                        num_staff += 1;
                    }
                }
                num_coverage.insert(staff.id.clone(), num_staff);
            }
        }

       num_coverage
    }

    pub fn calculate_number_horizontal_coverage_violation(
        &self,
        coverage: &HorizontalCoverage,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    ) -> i8{
        let mut num_coverage = self.calculate_number_horizontal_coverage_fulfill(&coverage, &week, schedule);
        let mut num_violation = 0;

        for (_, item) in num_coverage{
            if coverage.types.contains(&"at least".to_string()){
                num_violation +=  max(&coverage.desire_value - &item, 0);
            }
            else if coverage.types.contains(&"equal to".to_string()){
                num_violation +=  max(&coverage.desire_value - &item, &item -coverage.desire_value)
            }
            else if coverage.types.contains(&"at most".to_string()){
                num_violation +=  max(&item - &coverage.desire_value, 0)
            }
        }

        num_violation
    }

    pub fn calculate_number_staff_time_fulfill(
        &self,
        staff: &String,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    )-> f32{
        let mut time:f32 = 0.0;
        for day in 0..=6i8 {
            time += solution::get_duration(&schedule, &staff, date::convert_to_solution_hashmap_index(&day.clone(), &week), &self.input.shifts) as f32;
        }

        time
    }

    pub fn calculate_number_staff_day_fulfill(
        &self,
        staff: &String,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    ) -> f32{
        let mut working_day = 0.0;
        for day in 0..=6{
            let value = match solution::get_duration(&schedule, &staff, date::convert_to_solution_hashmap_index(&day.clone(), &week), &self.input.shifts) {
                8|7 => 1.0,
                4 => 0.5,
                _ => 0.0
            };

            working_day  += value;
        }

        working_day
    }

    pub fn constraint_violation(
        &self,
        constraint: &Constraint,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    )-> HashMap<String, f32>
    {
        let mut map= match constraint.id.as_str() {
            "exactly-staff-working-time" => {
                let mut temp_map :HashMap<String, f32> = HashMap::new();

                for staff in &self.input.staffs{
                    temp_map.insert(staff.id.clone(), self.calculate_number_staff_time_fulfill(&staff.id, &week, &schedule));
                }

                temp_map
            }

            "archive-0.5-day" =>{
                let mut temp_map :HashMap<String, f32> = HashMap::new();

                for staff in &constraint.staff_groups{
                    temp_map.insert(staff.clone(), self.calculate_number_staff_day_fulfill(&staff, &week, &schedule));
                }

                temp_map
            }

            "un-archive-0.5-day" =>{
                let mut temp_map :HashMap<String, f32> = HashMap::new();

                for staff in &constraint.staff_groups{
                    temp_map.insert(staff.clone(), self.calculate_number_staff_day_fulfill(&staff, &week, &schedule));
                }

                temp_map
            }

            _ => {HashMap::new()}
        };

        map
    }

    pub fn pattern_constraint_violation(
        &self,
        constraint: &PatternConstraint,
        week: &i8,
        schedule: &HashMap<String, HashMap<i8, String>>
    )-> HashMap<String, f32>{

        let mut temp_map :HashMap<String, f32> = HashMap::new();

        for staff_group in &constraint.staff_groups {
            for staff_ in &self.input.staff_groups.iter().find(|&x| x.id == *staff_group).unwrap().staff_list {
                let mut violation = 0;
                for pattern in &constraint.shift_patterns {
                    for day in 0..=6 - pattern.len() as i8 {
                        if solution::get_value(&schedule, &staff_, date::convert_to_solution_hashmap_index(&day, &week)).unwrap() == pattern[0] {
                            let mut boolean = true;

                            for index in 1..pattern.len() {
                                if solution::get_value(
                                    &schedule, &staff_,
                                    date::convert_to_solution_hashmap_index(&(&day + index as i8), &week)
                                ).unwrap() != pattern[index] {
                                    boolean = false;
                                    break;
                                }
                            }

                            if boolean {
                                violation += 1;
                            }
                        }
                    }
                }
                temp_map.insert(staff_.clone(), violation as f32);
            }
        }

        temp_map
    }
    /// violation constraint utils

    pub(crate) fn number_constraint_violation(&self, constraint: &InterfaceConstraint, schedule: &HashMap<String, HashMap<i8, String>>) -> i32{
        let bool_result = match constraint {
            InterfaceConstraint::Constraint(constraint_clone) => {
                let result_match = match constraint_clone.id.as_str() {
                    "exactly-staff-working-time" => {
                        let mut number_violation = 0;
                        for week in 1..=self.input.schedule_period {
                            for staff in &self.input.staffs {
                                number_violation += match self.calculate_number_staff_time_fulfill(&staff.id, &week, &schedule) {
                                    44.0 => { 0 }
                                    _ => { 1 }
                                } as i32;
                            }
                        }
                        number_violation
                    }

                    "archive-0.5-day" => {
                        let mut number_violation = 0;
                        for week in 1..=self.input.schedule_period {
                            for staff in &constraint_clone.staff_groups {
                                number_violation += match self.calculate_number_staff_day_fulfill(&staff, &week, &schedule) {
                                    5.5 => { 0 }
                                    _ => { 1 }
                                } as i32;
                            }
                        }
                        number_violation
                    }

                    "un-archive-0.5-day" => {
                        let mut number_violation = 0;
                        for week in 1..=self.input.schedule_period {
                            for staff in &constraint_clone.staff_groups {
                                number_violation += match self.calculate_number_staff_day_fulfill(&staff, &week, &schedule) {
                                    6.0 => { 0 }
                                    _ => { 1 }
                                } as i32;
                            }
                        }
                        number_violation
                    }

                    _ => {
                        {0}
                    }
                };

                result_match
            }
            InterfaceConstraint::HorizontalCoverage(constraint_clone) => {
                let mut number_violation = 0;
                for week in 1..= self.input.schedule_period{
                    number_violation += self.calculate_number_horizontal_coverage_violation(&constraint_clone, &week, & schedule) as i32;
                }

                number_violation
            }
            InterfaceConstraint::PatternConstraint(constraint_clone) => {
                let mut number_violation = 0;
                for week in 1..= self.input.schedule_period{
                    for week in 1..=self.input.schedule_period {
                        let map_number_violation = self.pattern_constraint_violation(&constraint_clone, &week, &schedule);

                        for (_, value) in map_number_violation{
                            number_violation += value as i32;
                        }
                    }
                }

                number_violation
            }

            _ => { 0 }
        };


        bool_result
    }

    pub(crate) fn list_number_constraint_violation(&self, list_constraint_upper_priority: &HashMap<i8, InterfaceConstraint>, schedule: &HashMap<String, HashMap<i8, String>>) -> HashMap<i8, i8>{
        let mut list_violation_temp_schedule: HashMap<i8, i8> = HashMap::new();
        for (index, cons) in *&list_constraint_upper_priority{
            list_violation_temp_schedule.insert(*index, self.number_constraint_violation(&cons, &schedule) as i8);
        }

        list_violation_temp_schedule
    }


    pub(crate) fn get_higher_priority_constraint(
        &self,
        current_constraint_priority: &i8,
        constraint_id: &String,
    ) -> HashMap<i8, InterfaceConstraint>
    {
        let mut map: HashMap<i8, InterfaceConstraint> = HashMap::new();

        for cons in &self.input.constraints.clone() {
            if cons.priority >= *current_constraint_priority && cons.id != *constraint_id {
                let conn = InterfaceConstraint::Constraint(cons.clone());
                map.insert(cons.priority, conn);
            }
        }

        for horizontal_constraint in &self.input.horizontal_coverages.clone() {
            if horizontal_constraint.priority >= *current_constraint_priority {
                let conn = InterfaceConstraint::HorizontalCoverage(horizontal_constraint.clone());
                map.insert(horizontal_constraint.priority, conn);
            }
        }

        for pattern_constraint in &self.input.pattern_constraints.clone() {
            if pattern_constraint.priority >= *current_constraint_priority {
                let conn = InterfaceConstraint::PatternConstraint(pattern_constraint.clone());
                map.insert(pattern_constraint.priority, conn);
            }
        }

        map
    }

    pub(crate) fn is_make_upper_constraint_worse(&self, old_constraint_violation_list: &HashMap<i8, i8>, new_constraint_violation_list: &HashMap<i8, i8>) -> bool{
        for index in 10..=1 {
            if old_constraint_violation_list.get(&index).is_some() {
                if old_constraint_violation_list.get(&index) > new_constraint_violation_list.get(&index) {
                    return false;
                }

                else if old_constraint_violation_list.get(&index) < new_constraint_violation_list.get(&index){
                    return true;
                }
            }
        }

        false
    }
}