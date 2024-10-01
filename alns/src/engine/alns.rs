use std::collections::HashMap;
use crate::coverage::coverage::Coverage;
use crate::input::input::InputData;
use crate::utils::random;
use std::hash::Hash;
use std::ptr::null;
use rand::random;
use crate::staff::staff::Staff;

pub struct Alns {
    max_iteration: i32,
    delta_e: f32,
    alpha: f32,
    temperature: f32,
    operator_score: [i8; 5],

    operator_weight: [i8; 5],
    operator_time: [i8; 5],
    operator_probabilities: [i8; 5],
    solution: HashMap<String,HashMap<i8,String>>,
    input: InputData
}

impl Alns {

    pub fn init(input_data: InputData) -> Self {
        Self {
            max_iteration: 1000,
            delta_e: 0.05,
            alpha: 100.0,
            temperature: 100.0,
            operator_score: [0; 5],
            operator_weight: [0; 5],
            operator_time: [0; 5],
            operator_probabilities: [0; 5],
            solution: HashMap::new(),
            input: input_data
        }
    }

    fn initial_solution(&self) -> HashMap<String, HashMap<i8, String>> {
        let mut initial_solution: HashMap<String, HashMap<i8, String>> = HashMap::new();

        for staff in &self.input.staffs {
            let staff_id = &staff.id;
            let mut inner_map: HashMap<i8, String> = HashMap::new();

            for index in 0..self.input.schedule_period * 7 {
                if (index + 1) % 7 == 0 {
                    inner_map.insert(index, "DO".to_string());
                } else {
                    inner_map.insert(index, "".to_string());
                }
            }

            initial_solution.insert(staff_id.to_string(), inner_map);
        }

        for staff in &self.input.staffs {
            for index in 0..self.input.schedule_period * 7 {
                if (initial_solution[&staff.id][&index] != "DO".to_string()) {
                    for coverage in &self.input.coverages {
                        if coverage.desire_value > self.coverage_calculate(index, &coverage, &initial_solution) {
                            if let Some(inner_map) = initial_solution.get_mut(&staff.id) {
                                inner_map.insert(index as i8, coverage.shift_random());
                            }
                        }
                    }
                }
            }
        }

        self.adjust_for_public_holiday(initial_solution)
    }

    fn is_violation_core_day(&self, schedule: &HashMap<String, HashMap<i8, String>>, staff: &String, index: i8) -> bool{
        let bool_value = match schedule[staff].get(&index).unwrap().as_str() {
            "DO"| "PH" => true,
            _ => false
        };

        bool_value
    }

    fn is_a_day(&self, schedule: &HashMap<String, HashMap<i8, String>>, staff: &String, index: i8, shift_info: &str) -> bool{
        let bool_value = match schedule[staff].get(&index).unwrap().as_str() {
            shift_info => true,
            _ => false
        };

        bool_value
    }

    fn is_leap_year()->bool{
        false
    }

    fn adjust_for_public_holiday(&self, mut schedule: HashMap<String, HashMap<i8, String>>) -> HashMap<String, HashMap<i8, String>> {
        for staff in &self.input.staffs {
            let all_staff_groups = &self.input.staff_groups;

            if all_staff_groups.iter().filter(|x| x.id == "OPH").any(|x| x.staff_list.contains(&staff.id)) {
                let mut current_month = self.input.start_date.month;
                let mut total_processing_day = 0;
                let mut current_day = self.input.start_date.day;
                let total_days = self.input.schedule_period * 7;

                while total_processing_day < total_days {
                    let day_in_month = match current_month {
                        2 => if Self::is_leap_year() { 29 } else { 28 },
                        4 | 6 | 9 | 11 => 30,
                        _ => 31,
                    };

                    if current_day > day_in_month {
                        current_day = 1;
                        current_month += 1;
                        if current_month > 12 {
                            current_month = 1;
                        }
                    }

                    if self.input.public_holidays.iter().any(|x| x.day == current_day && x.month == current_month) {
                        if let Some(inner_map) = schedule.get_mut(&staff.id) {
                            inner_map.insert(total_processing_day, "PH".to_string());
                        }
                    }

                    current_day += 1;
                    total_processing_day += 1;
                }
            }
        }
        schedule
    }


    fn coverage_calculate(&self, day: i8, coverage: &Coverage, schedule: &HashMap<String, HashMap<i8, String>>) -> i8 {
        let mut number_coverage_fulfill  = 0;
        for staff in &self.input.staffs {
            if coverage.shift.contains(&schedule[&staff.id][&day]) {
                number_coverage_fulfill += 1;
            }
        }
        number_coverage_fulfill
    }

    fn route_wheel(&self, index: i8){

    }

    fn random_destroy_solution(&self, schedule: &mut HashMap<String, HashMap<i8, String>>) -> HashMap<String, HashMap<i8, String>> {

        let mut random_key = *random::random_choice(&vec![0, 1, 2, 3, 4, 5, 6]);
        let mut random_week = random::random_choice_from_range(1usize, *&self.input.schedule_period as usize);
        let mut random_staff = random::random_choice(&self.input.staffs);
        while !self.is_violation_core_day(*&schedule, &random_staff.id, (&random_key + 7 * (&random_week - 1)) as i8) {
            random_key = *random::random_choice(&vec![0,1,2,3,4,5,6]);
            random_week = random::random_choice_from_range(1usize, *&self.input.schedule_period as usize);
            random_staff = random::random_choice(&self.input.staffs);
        }

        if let Some(inner_map) = schedule.get_mut(&random_staff.id){
            inner_map.insert(((random_key + 7 * (random_week - 1)) as i8), "".to_string());
        }
        schedule.to_owned()
    }

    fn repair_solution<'a>(&self, schedule: &'a HashMap<String,HashMap<i8, String>>) -> &'a HashMap<String, HashMap<i8, String>> {

        for staff in &self.input.staffs {
            for week in 1..= self.input.schedule_period{
                for day in 0..=6{
                    if self.is_a_day(*&schedule, &staff.id, &day + 7 * (&week - 1), ""){


                    }
                }
            }

        }
        schedule
    }

    fn simulate_annealing(&self, schedule: &HashMap<String,HashMap<i8, String>>, next_schedule: &HashMap<String,HashMap<i8, String>>){

    }

    fn shake_and_repair(&self, schedule: &HashMap<String,HashMap<i8, String>>, operator_index: &i32) -> &HashMap<String, HashMap<i8, String>> {

        schedule
    }


    pub fn run_iteration(&mut self){
        let mut current_solution = self.initial_solution();
        self.solution = current_solution;
        for iter_num in 1..= self.max_iteration{
            let operator_index = self.routeWheel(&iter_num);
            let next_solution = self.shake_and_repair(&current_solution, operator_index);
            current_solution = self.simulate_annealing(&current_solution, &next_solution)
            if (calculate.totalScore(current_solution) > calculate.totalScore(this.bestSolution)){
                this.bestSolution = deepCopySolution(current_solution)
            }
        }
        self.print_solution();
    }

    pub fn print_solution(&self){
        println!("[solution]");
        for (key, value) in &self.solution {
            println!("{}: {:?}", key, value);
        }
    }

}