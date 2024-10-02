use std::cmp::max;
use std::collections::HashMap;
use std::thread::sleep;
use tokio::time::interval;
use crate::input::input::InputData;
use crate::coverage::coverage::Coverage;
use crate::coverage::horizontal_coverage::HorizontalCoverage;

pub struct Rule<'a> {
    hard_violation: HashMap<String, HashMap<i32, i32>>,
    soft_violation: HashMap<String, HashMap<i32, i32>>,
    input: &'a InputData
}

impl<'a> Rule<'a> {

    pub fn new(input_data: &'a InputData) -> Rule {
        Self{
            hard_violation: HashMap::new(),
            soft_violation: HashMap::new(),
            input: input_data
        }
    }


    pub fn calculate_number_coverage_fulfill(&self, coverage: &Coverage, week: &i8, schedule: &HashMap<String, HashMap<i8, String>>) -> i8
    {
        let mut num_violation = 0;

        for staff_group_id in &coverage.staff_groups {
            let staff_group = &self.input.staff_groups.iter().find(|&x| x.id == *staff_group_id).unwrap();
            for staff in &staff_group.staff_list{
                //schedule[staff].get(&index).unwrap().as_str()
                if coverage.shift.contains(&schedule[*&staff].get(&(&coverage.day -1 + 7 * (week - 1))).unwrap()) {
                    num_violation += 1;
                }
            }
        }

        num_violation
    }

    pub fn calculate_number_coverage_violation(&self, coverage: &Coverage, week: &i8, schedule: &HashMap<String, HashMap<i8, String>>) -> i8{
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

    pub fn calculate_number_horizontal_coverage_fulfill(&self, coverage: &HorizontalCoverage, week: &i8, schedule: &HashMap<String, HashMap<i8, String>>) -> HashMap<String, i8>{
        let mut num_coverage : HashMap<String, i8> = HashMap::new();

        if coverage.staffs.contains(&"all_staff".to_string()){
            for staff in &self.input.staffs{
                let mut num_staff = 0;
                for day in &coverage.days{
                    if coverage.shifts.contains(schedule[&staff.id].get(&(day - 1 + 7 * (week - 1))).unwrap()) {
                        num_staff += 1;
                    }
                }
                num_coverage.insert(staff.id.clone(), num_staff);
            }
        }

       num_coverage
    }

    pub fn calculate_number_horizontal_coverage_violation(&self, coverage: &HorizontalCoverage, week: &i8, schedule: &HashMap<String, HashMap<i8, String>>) -> i8{
        let mut num_coverage = self.calculate_number_horizontal_coverage_fulfill(&coverage, &week, schedule);
        let mut num_violation = 0;

        for (_, item) in num_coverage{
            if coverage.types.contains(&"at least".to_string()){
                num_violation +=  max(&coverage.desire_value - item, 0);
            }
            else if coverage.types.contains(&"equal to".to_string()){
                num_violation +=  max(&coverage.desire_value - item, item -coverage.desire_value)
            }
            else if coverage.types.contains(&"at most".to_string()){
                num_violation +=  max(item - &coverage.desire_value, 0)
            }
        }

        num_violation
    }
}