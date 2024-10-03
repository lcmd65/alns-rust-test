use std::collections::HashMap;
use crate::coverage::coverage::Coverage;
use crate::engine::cost::Score;
use crate::input::input::InputData;
use crate::utils::random;
use crate::utils::hashing;
use crate::utils::date;
use crate::solution::solution;
use crate::utils::to_excel;
use std::hash::Hash;
use rand::{random, thread_rng, Rng};

pub struct Alns<'a> {
    max_iteration: i32,
    delta_e: f32,
    alpha: f32,
    limit: f32,
    temperature: f32,
    operator_score: [f32; 5],
    operator_weight: [f32; 5],
    operator_time: [f32; 5],
    operator_probabilities: [f32; 5],
    solution: HashMap<String,HashMap<i8,String>>,
    input: &'a InputData,
    score: Score<'a>
}

impl<'a> Alns<'a> {

    pub fn new(input_data: &'a InputData) -> Self {
        let alns = Self {
            max_iteration: 1000,
            delta_e: 0.0,
            limit: 1e-100,
            alpha: 0.95,
            temperature: 100.0,
            operator_score: [0.1, 0.1, 0.1, 0.5, 0.2],
            operator_weight: [0.0; 5],
            operator_time: [1.0; 5],
            operator_probabilities: [0.0; 5],
            solution: HashMap::new(),
            score: Score::new(&input_data),
            input: &input_data
        };
        alns
    }

    fn update_weight(&mut self){
        for index in 0..=4{
            if self.operator_weight[index] == 0.0 {
                self.operator_weight[index] =
                    0.4 +
                    0.6 * self.operator_score[index] / self.operator_time[index];
            }
            else {
                self.operator_weight[index] =
                    0.4 * self.operator_weight[index] +
                    0.6 * self.operator_score[index] / self.operator_time[index];
            }
        }
    }

    fn reset_weight(&mut self){
        for index in 0..=4{
            self.operator_weight[index] =
                0.4 +
                0.6 * self.operator_score[index] / self.operator_time[index];
        }
    }

    fn initial_solution(&self) -> HashMap<String, HashMap<i8, String>> {

        let mut initial_solution: HashMap<String, HashMap<i8, String>> = HashMap::new();
        for staff in &self.input.staffs {
            let staff_id = &staff.id;
            let mut inner_map: HashMap<i8, String> = HashMap::new();

            for index in 0.. &self.input.schedule_period * 7 {
                if (index + 1) % 7 == 0 {
                    inner_map.insert(index, "DO".to_string());
                } else {
                    inner_map.insert(index, "".to_string());
                }
            }

            initial_solution.insert(staff_id.to_string(), inner_map);
        }

        for staff in &self.input.staffs {
            for index in 0..&self.input.schedule_period * 7 {
                if (initial_solution[&staff.id][&index] != "DO".to_string()) {
                    for coverage in &self.input.coverages {
                        if coverage.desire_value >
                            self.coverage_calculate(
                                index,
                                &coverage,
                                &initial_solution
                            )
                        {
                            if let Some(inner_map) = initial_solution.get_mut(&staff.id) {
                                inner_map.insert(index, coverage.shift_random());
                            }
                        }
                    }
                }
            }
        }

        for staff in &self.input.staffs {
            for week in 1..= self.input.schedule_period{
                for day in 0..=6{
                    if solution::is_a_shift(
                        &initial_solution,
                        &staff.id,
                        &day + 7 * (&week - 1),
                        "".to_string())
                    {
                        let mut random_shift = random::random_choice(&self.input.shifts);

                        while(*&random_shift.id == "DO".to_string() || *&random_shift.id == "PH".to_string()) {
                            random_shift = random::random_choice(&self.input.shifts);
                        }

                        if let Some(inner_map) = initial_solution.get_mut(&staff.id) {
                            inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), random_shift.id.to_string());
                        }
                    }
                }
            }
        }

        self.adjust_for_public_holiday(initial_solution)
    }

    fn adjust_for_public_holiday(
        &self,
        mut schedule: HashMap<String, HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>>
    {
        for staff in &self.input.staffs {
            let all_staff_groups = &self.input.staff_groups;

            if all_staff_groups
                .iter()
                .filter(|x| x.id == "OPH")
                .any(|x| x.staff_list.contains(&staff.id))
            {
                let mut current_month = self.input.start_date.month;
                let mut total_processing_day = 0;
                let mut current_day = self.input.start_date.day;
                let total_days = self.input.schedule_period * 7;

                while total_processing_day < total_days {
                    let day_in_month = match current_month {
                        2 => if date::is_leap_year() { 29 } else { 28 },
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

                    if self.input.public_holidays
                        .iter()
                        .any(|x| x.day == current_day && x.month == current_month)
                    {
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

    fn coverage_calculate(
        &self,
        day: i8,
        coverage: &Coverage,
        schedule: &HashMap<String, HashMap<i8, String>>
    ) -> i8
    {
        let mut number_coverage_fulfill  = 0;
        for staff in &self.input.staffs {
            if coverage.shift.contains(&schedule[&staff.id][&day]) {
                number_coverage_fulfill += 1;
            }
        }
        number_coverage_fulfill
    }

    fn route_wheel(
        &mut self,
        iter: i32
    ) -> i8
    {
        if iter % 400 == 0{ self.reset_weight() }
        else { self.update_weight() }

        let rand: f32 = random();
        let mut sum = 0.0;

        for index in 0.. self.operator_weight.len(){
            sum += &self.operator_weight[index];
        }

        self.operator_probabilities[0] = &self.operator_weight[0]/sum;

        for index in 1.. self.operator_weight.len(){
            self.operator_probabilities[index] =  &self.operator_probabilities[index-1] +  &self.operator_weight[index]/sum;
        }

        let mut choose_value:i8 = 0;
        if (rand <= self.operator_probabilities[0]){
            choose_value = 0;
        }
        else{
            for index in 1 ..= self.operator_weight.len() {
                if rand > self.operator_probabilities[index - 1] && rand <= self.operator_probabilities[index] {
                    choose_value = index as i8
                }
            }
        }

        choose_value
    }

    fn random_swap_staff_shift(
        &self,
        schedule: &mut HashMap<String, HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>> {
        let mut random_key = *random::random_choice(&vec![0, 1, 2, 3, 4, 5, 6]);
        let mut random_week = random::random_choice_from_range(1usize, *&self.input.schedule_period as usize);
        let mut random_staff = random::random_choice(&self.input.staffs);
        while !solution::is_violation_core_day(
            &schedule,
            &random_staff.id,
            (&random_key + 7 * (&random_week - 1)) as i8
        ) {
            random_key = *random::random_choice(&vec![0,1,2,3,4,5,6]);
            random_week = random::random_choice_from_range(1usize, *&self.input.schedule_period as usize);
            random_staff = random::random_choice(&self.input.staffs);
        }

        let mut random_shift = random::random_choice(&self.input.shifts);
        if random_staff.work_days == 5.5 {
            while ["M2", "A2", "PH"].contains(&&*random_shift.id.to_string()) {
                random_shift = random::random_choice(&self.input.shifts);
            }
        } else {
            while["M3", "PH"].contains(&&*random_shift.id.to_string()) {
                random_shift = random::random_choice(&self.input.shifts);
            }
        }

        if let Some(inner_map) = schedule.get_mut(&random_staff.id){
            inner_map.insert(((random_key + 7 * (random_week - 1)) as i8), random_shift.id.to_string());
        }
        schedule.clone()
    }

    fn simulate_annealing(
        &mut self,
        schedule: &HashMap<String, HashMap<i8, String>>,
        next_schedule: &HashMap<String,HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>> {
        self.delta_e = self.score.calculate_total_score(&schedule) - self.score.calculate_total_score(&next_schedule);
        if (self.delta_e < 0.0){
            return next_schedule.clone()
        }
        else {
            if self.temperature < self.limit {
                return schedule.clone()
            }
            let probability = (self.delta_e / self.temperature).exp();
            let acceptance_variable = random::random_choice_from_range_double(0.0, 1.0);

            self.temperature *= self.alpha;
            if (probability < acceptance_variable) {
                return next_schedule.clone()
            }
        }

        schedule.clone()
    }

    fn greedy_coverage_enhancement(
        &self,
        schedule: &mut HashMap<String, HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>> {
        let mut next_schedule: HashMap<String, HashMap<i8, String>> = HashMap::new();
        for coverage in &self.input.coverages {
            for staff_group_id in &coverage.staff_groups {
                let staff_group = &self.input.staff_groups.iter().find(|&x| x.id == *staff_group_id).unwrap();
                for staff in &staff_group.staff_list {
                    for week in 1..self.input.schedule_period {
                        for shift in coverage.shift.clone() {
                            next_schedule = schedule.clone();
                            if let Some(inner_map) = next_schedule.get_mut(&staff.clone()) {
                                inner_map.insert(date::convert_to_solution_hashmap_index(&(&coverage.day - 1), &week), shift);
                            }
                            if self.score.calculate_coverage_score(&schedule) < self.score.calculate_coverage_score(&next_schedule){
                                return next_schedule.clone()
                            }
                        }
                    }
                }
            }
        }

        schedule.clone()
    }

    fn greedy_horizontal_coverage_enhancement(
        &self,
        schedule: &mut HashMap<String, HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>>
    {
        let mut next_schedule: HashMap<String, HashMap<i8, String>> = HashMap::new();
        for horizontal_coverage in &self.input.horizontal_coverages {
            for staff in &self.input.staffs {
                for week in 1..self.input.schedule_period {
                    for day in horizontal_coverage.days.clone() {
                        for shift in horizontal_coverage.shifts.clone() {
                            next_schedule = schedule.clone();
                            if let Some(inner_map) = next_schedule.get_mut(&staff.id.clone()) {
                                inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), shift);
                            }
                            if self.score.calculate_horizontal_coverage_score(&schedule) < self.score.calculate_horizontal_coverage_score(&next_schedule) {
                                return next_schedule.clone()
                            }
                        }
                    }
                }
            }
        }

        schedule.clone()
    }

    fn greedy_swap_staff_shift_enhancement(
        &self,
        schedule: &mut HashMap<String, HashMap<i8, String>>
    )-> HashMap<String, HashMap<i8, String>>
    {
        let mut next_schedule = schedule.clone();
        for staff in &self.input.staffs{
            for week in 1..= *&self.input.schedule_period{
                for day in 0..6{
                    if !solution::is_violation_public_holiday(&schedule, &staff.id, *&day + 7*(&week - 1)) {
                        for day_next in (&day.clone() + 1)..=6 {
                            if !solution::is_violation_public_holiday(
                                &schedule,
                                &staff.id,
                                date::convert_to_solution_hashmap_index(&day_next, &week)
                            ) &&
                                !solution::is_a_shift(
                                    &schedule,&staff.id,
                                    date::convert_to_solution_hashmap_index(&day, &week),
                                    solution::get_value(
                                        &schedule,
                                        &staff.id,
                                        date::convert_to_solution_hashmap_index(&day_next, &week)
                                    ).unwrap().to_string()
                                )
                            {
                                next_schedule = schedule.clone();

                                if let Some(inner_map) = next_schedule.get_mut(&staff.id) {
                                    if let Some(temp_shift) = inner_map.get(&date::convert_to_solution_hashmap_index(&day, &week)).cloned() {
                                        inner_map.insert(
                                            date::convert_to_solution_hashmap_index(&day, &week),
                                            inner_map.get(&date::convert_to_solution_hashmap_index(&day_next, &week))
                                                .unwrap()
                                                .clone()
                                        );
                                        inner_map.insert(date::convert_to_solution_hashmap_index(&day_next, &week), temp_shift);
                                    }
                                }

                                if self.score.calculate_total_score(&schedule) < self.score.calculate_total_score(&next_schedule) {
                                    return next_schedule;
                                }
                            }
                        }
                    }
                }
            }
        }

        next_schedule
    }

    fn shake_and_repair(
        &self,
        schedule: &mut HashMap<String,HashMap<i8, String>>, operator_index: i8
    ) -> HashMap<String, HashMap<i8, String>> {
        let result = match operator_index{
            0 => self.random_swap_staff_shift(schedule),
            1 => self.greedy_coverage_enhancement(schedule),
            2 => self.greedy_horizontal_coverage_enhancement(schedule),
            3 => self.greedy_swap_staff_shift_enhancement(schedule),
            4 => self.greedy_swap_staff_shift_enhancement(schedule),
            _ => schedule.clone()
        };

        result
    }

    pub fn run_iteration(&mut self){
        let mut current_solution = self.initial_solution();
        self.solution = current_solution.clone();

        for iter_num in 1..= self.max_iteration{
            let operator_index = self.route_wheel(iter_num);
            self.operator_time[operator_index as usize] += 1.0;

            let next_solution = self.shake_and_repair(&mut current_solution, operator_index);
            current_solution = self.simulate_annealing(&current_solution, &next_solution);
            println!("{}", self.score.calculate_total_score(&current_solution));

            if (self.score.calculate_total_score(&current_solution) > self.score.calculate_total_score(&self.solution)){
                self.solution = current_solution.clone();
            }
        }

        self.print_solution();
    }

    pub fn print_solution(&self){
        to_excel::write_hashmap_to_excel(&self.solution.clone(),"src/output/output.xlsx");
        println!("[solution]");
        for (key, value) in &self.solution.clone(){
            println!("{}: {:?}", key, value);
        }

        let score_coverage = self.score.calculate_coverage_score(&self.solution);
        let h_score_coverage = self.score.calculate_horizontal_coverage_score(&self.solution);
        let score_constraint = self.score.calculate_constraint_score(&self.solution);
        let score_pattern_constraint = self.score.calculate_pattern_constraint_score(&self.solution);
        println!("[coverage score]: {}", score_coverage);
        println!("[horizontal coverage score]: {}", h_score_coverage);
        println!("[constraint score]: {}", score_constraint);
        println!("[pattern-constraint score]: {}", score_pattern_constraint);
    }
}