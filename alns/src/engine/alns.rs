use std::any::Any;
use std::collections::{HashMap, HashSet};
use crate::coverage::coverage::Coverage;
use crate::coverage::horizontal_coverage::HorizontalCoverage;
use crate::constraint::pattern_constraint::PatternConstraint;
use crate::constraint::constraint::Constraint;
use crate::engine::cost::Score;
use crate::input::input::InputData;
use crate::utils::random;
use crate::utils::hashing;
use crate::utils::date;
use crate::solution::solution;
use crate::utils::to_excel;
use std::hash::Hash;
use std::thread::current;
use rand::{random, thread_rng, Rng};
use crate::constraint::InterfaceConstraint;
use crate::coverage::horizontal_coverage;
use crate::staff::staff::Staff;
use crate::violation::rule::Rule;

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
    score: Score<'a>,
    rule: Rule<'a>
}

impl<'a> Alns<'a> {

    pub fn new(input_data: &'a InputData) -> Self {
        let alns = Self {
            max_iteration: 1000,
            delta_e: 0.0,
            limit: 1e-100,
            alpha: 0.95,
            temperature: 100.0,
            operator_score: [0.2; 5],
            operator_weight: [0.0; 5],
            operator_time: [1.0; 5],
            operator_probabilities: [0.0; 5],
            solution: HashMap::new(),
            score: Score::new(&input_data),
            rule: Rule::new(&input_data),
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

    fn initial_solution(&self) -> HashMap<String, HashMap<i8, String>> {

        let mut initial_solution: HashMap<String, HashMap<i8, String>> = HashMap::new();
        for staff in &self.input.staffs {
            let staff_id = &staff.id;
            let mut inner_map: HashMap<i8, String> = HashMap::new();

            for index in 0..&self.input.schedule_period * 7 {
                if (&index + 1) % 7 == 0 {
                    inner_map.insert(index.clone(), "DO".to_string());
                } else {
                    inner_map.insert(index.clone(), "".to_string());
                }
            }
            initial_solution.insert(staff_id.to_string(), inner_map);
        }
        for staff in &self.input.staffs {
            for index in 0.. &self.input.schedule_period * 7 {
                if solution::is_a_shift(&initial_solution, &staff.id, index, "".to_string()) {
                    for coverage in &self.input.coverages {
                        if coverage.desire_value >
                            self.coverage_calculate(
                                index.clone(),
                                &coverage,
                                &initial_solution
                            )
                        {
                            if let Some(inner_map) = initial_solution.get_mut(&staff.id) {
                                inner_map.insert(index.clone(), coverage.shift_random());
                            }
                        }
                    }

                    if solution::is_a_shift(&initial_solution, &staff.id, index, "".to_string()) {
                        let mut random_shift = random::random_choice(&self.input.shifts);
                        while (*&random_shift.id == "DO".to_string() || *&random_shift.id == "PH".to_string()) {
                            random_shift = random::random_choice(&self.input.shifts);
                        }

                        if let Some(inner_map) = initial_solution.get_mut(&staff.id) {
                            inner_map.insert(index.clone(), random_shift.id.to_string());
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

    fn route_wheel(
        &mut self,
        iter: i32
    ) -> i8
    {
        if iter % 400 == 0{ self.reset_weight() }
        else { self.update_weight() }

        let rand: f32 = random();
        let sum: f32 = self.operator_weight.iter().sum();

        self.operator_probabilities[0] = &self.operator_weight[0]/sum;

        for index in 1.. self.operator_weight.len(){
            self.operator_probabilities[index] =
                &self.operator_probabilities[index-1] +
                &self.operator_weight[index]/sum;
        }

        let mut choose_value:i8 = 0;
        if (rand <= self.operator_probabilities[0]){
            choose_value = 0;
        } else{
            for index in 1 ..= self.operator_weight.len() {
                if rand > self.operator_probabilities[index - 1] &&
                    rand <= self.operator_probabilities[index]
                {
                    choose_value = index as i8
                }
            }
        }

        choose_value
    }

    fn random_swap_staff_shift(
        &self,
        schedule: &mut HashMap<String, HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>>
    {
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

    fn greedy_coverage_enhancement(
        &self,
        schedule: &mut HashMap<String, HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>> {
        let mut next_schedule: HashMap<String, HashMap<i8, String>> = HashMap::new();
        for coverage in &self.input.coverages {
            for week in 1..self.input.schedule_period {
                let fulfill_value = self.rule.calculate_number_coverage_fulfill(&coverage, &week, &schedule);
                if fulfill_value < coverage.desire_value{
                    if coverage.types.contains(&"at least".to_string()) || coverage.types.contains(&"equal to".to_string()) {
                        for staff_group_id in &coverage.staff_groups {
                            let staff_group = &self.input.staff_groups.iter().find(|&x| x.id == *staff_group_id).unwrap();
                            for staff in &staff_group.staff_list {
                                if !coverage.shift.clone()
                                    .contains(&solution::get_value(
                                        &schedule,
                                        &staff,
                                        date::convert_to_solution_hashmap_index(&(&coverage.day - 1), &week)
                                    ).unwrap().to_string()) &&
                                    !["PH", "DO"].contains(&&**&solution::get_value(
                                        &schedule,
                                        &staff,
                                        date::convert_to_solution_hashmap_index(&(&coverage.day - 1), &week)
                                    ).unwrap().to_string())
                                {
                                    for shift in coverage.shift.clone() {
                                        next_schedule = schedule.clone();
                                        if let Some(inner_map) = next_schedule.get_mut(&staff.clone()) {
                                            inner_map.insert(date::convert_to_solution_hashmap_index(&(&coverage.day - 1), &week), shift);
                                        }
                                        if self.score.calculate_coverage_score(&schedule) < self.score.calculate_coverage_score(&next_schedule) {
                                            return next_schedule.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else if fulfill_value > coverage.desire_value {
                    if coverage.types.contains(&"equal to".to_string()) || coverage.types.contains(&"at most".to_string()){
                        for shift in &self.input.shifts{
                            if !coverage.shift.contains(&shift.id) &&
                                !["PH", "DO"].contains(&&**&shift.id.clone())
                            {
                                for staff_group_id in &coverage.staff_groups {
                                    let staff_group = &self.input.staff_groups.iter().find(|&x| x.id == *staff_group_id).unwrap();
                                    for staff in &staff_group.staff_list {
                                        next_schedule = schedule.clone();
                                        if let Some(inner_map) = next_schedule.get_mut(&staff.clone()) {
                                            inner_map.insert(date::convert_to_solution_hashmap_index(&(&coverage.day - 1), &week), shift.id.clone());
                                        }
                                        if self.score.calculate_coverage_score(&schedule) < self.score.calculate_coverage_score(&next_schedule) {
                                            return next_schedule.clone()
                                        }
                                    }
                                }
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

                    let fulfill_map = self.rule.calculate_number_horizontal_coverage_fulfill(&horizontal_coverage, &week, &schedule);
                    if horizontal_coverage.types.contains(&"equal to".to_string()) {
                        for (staff_id, value) in fulfill_map {
                            if value > horizontal_coverage.desire_value {
                                for day in horizontal_coverage.days.clone() {
                                    if horizontal_coverage.shifts.contains(
                                        &solution::get_value(&schedule, &staff_id, date::convert_to_solution_hashmap_index(&day, &week))
                                            .unwrap()
                                            .to_string()
                                    ) {
                                        for new_shift in &self.input.shifts{
                                            if !horizontal_coverage.shifts.contains(&new_shift.id) &&
                                                !["PH", "DO"].contains(&&*new_shift.id)
                                            {
                                                next_schedule = schedule.clone();
                                                if let Some(inner_map) = next_schedule.get_mut(&staff.id.clone()) {
                                                    inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), new_shift.id.clone());
                                                }
                                                if self.score.calculate_horizontal_coverage_score(&schedule)
                                                    < self.score.calculate_horizontal_coverage_score(&next_schedule) {
                                                    return next_schedule.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            else if value < horizontal_coverage.desire_value{
                                for day in horizontal_coverage.days.clone() {
                                    if !["PH", "DO"].contains(&&*solution::get_value(&schedule, &staff.id, day.clone())
                                        .unwrap()
                                        .to_string()
                                    ) && !horizontal_coverage.shifts.contains(&solution::get_value(&schedule, &staff.id, day.clone())
                                            .unwrap()
                                            .to_string()
                                    ) {
                                        for shift in horizontal_coverage.shifts.clone() {
                                            next_schedule = schedule.clone();
                                            if let Some(inner_map) = next_schedule.get_mut(&staff.id.clone()) {
                                                inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), shift);
                                            }
                                            if self.score.calculate_horizontal_coverage_score(&schedule)
                                                < self.score.calculate_horizontal_coverage_score(&next_schedule) {
                                                return next_schedule.clone()
                                            }
                                        }
                                    }
                                }
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

                                if self.score.calculate_coverage_score(&schedule) < self.score.calculate_coverage_score(&next_schedule) {
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


    fn hard_fix_constraint_violation(&self, schedule: &HashMap<String, HashMap<i8, String>>) -> HashMap<String, HashMap<i8, String>>{

        schedule.clone()
    }

    fn greedy_fix_coverage_violation(&self, schedule: &mut HashMap<String, HashMap<i8, String>>) -> HashMap<String, HashMap<i8, String>>{

        schedule.clone()
    }

    fn greedy_fix_constraint_violation(&self, schedule: &mut HashMap<String, HashMap<i8, String>>) -> HashMap<String, HashMap<i8, String>> {
        let mut next_temp_schedule = schedule.clone();

        for priority in (1..=10).rev() {
            for constraint in &self.input.constraints{
                if constraint.priority == priority{
                    let list_constraint_upper_priority = self.rule.get_higher_priority_constraint(&constraint.priority, &&constraint.id);
                    let list_violation  = self.rule.list_number_constraint_violation(&list_constraint_upper_priority, &schedule);

                    match constraint.id.as_str() {
                        "exactly-staff-working-time" => {
                            for week in 1..= *&self.input.schedule_period {
                                let map_temp_violation = self.rule.constraint_violation (
                                    &constraint,
                                    &week,
                                    &schedule
                                ) ;

                                for (staff_, violation) in map_temp_violation {
                                    if violation != 44.0 {
                                        if *&self.input.staffs
                                            .iter()
                                            .find(|&x| x.id == staff_)
                                            .unwrap()
                                            .work_days
                                            .clone() == 5.5
                                        {
                                            for day in 0..=6 {
                                                if ["M2", "A2"].contains(&solution::get_value(
                                                    &next_temp_schedule,
                                                    &staff_,
                                                    date::convert_to_solution_hashmap_index(&day, &week))
                                                    .unwrap()
                                                    .as_str()
                                                ) {
                                                    if let Some(inner_map) = next_temp_schedule.get_mut(&staff_) {
                                                        inner_map.insert(
                                                            date::convert_to_solution_hashmap_index(&day, &week),
                                                            match solution::get_value(
                                                                &schedule,
                                                                &staff_,
                                                                date::convert_to_solution_hashmap_index(&day, &week)).unwrap().as_str()
                                                            {
                                                                "M2" => { "M1".to_string() }
                                                                "A2" => { "A1".to_string() }
                                                                _ => {
                                                                    random::random_choice(&vec!["M1", "A1"]).to_string()
                                                                }
                                                            }
                                                        );
                                                    };
                                                }
                                            }

                                            let mut counting_duration_day: HashMap<i8, i8> = HashMap::new();
                                            for index in vec![0, 4, 7, 8] {
                                                counting_duration_day.insert(index, 0);
                                            }

                                            for day in 0..=6 {
                                                let current_shift = solution::get_value(
                                                    &next_temp_schedule,
                                                    &staff_,
                                                    date::convert_to_solution_hashmap_index(&day, &week)
                                                );

                                                let current_shift_duration = &self.input.shifts
                                                    .iter()
                                                    .find(|&x| x.id.as_str() == &current_shift.clone().unwrap())
                                                    .unwrap()
                                                    .duration;

                                                counting_duration_day.insert(*current_shift_duration, counting_duration_day.get(current_shift_duration).unwrap() + 1);
                                            }

                                            if counting_duration_day[&0] > 1 {
                                                let mut num = 0;
                                                while (num < counting_duration_day[&4] - 1) {
                                                    for day in 0..=6 {
                                                        if solution::get_value(&next_temp_schedule, &staff_, day).unwrap() == "DO" {
                                                            if let Some(inner_map) = next_temp_schedule.get_mut(&staff_) {
                                                                inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), "M1".to_string());
                                                            }
                                                            num += 1;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }

                                            if counting_duration_day[&4] == 0 {
                                                let mut list_next_schedule: Vec<HashMap<String, HashMap<i8, String>>> = Vec::new();
                                                for day in 0..=6 {
                                                    let shift_list = self.input.shifts.iter().filter(|&x| x.duration == 4);
                                                    for shift in shift_list {
                                                        let mut next_temp_temp_schedule = next_temp_schedule.clone();
                                                        if let Some(inner_map) = next_temp_temp_schedule.get_mut(&staff_) {
                                                            inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), shift.id.clone());
                                                        }
                                                        list_next_schedule.insert(
                                                            day as usize,
                                                            next_temp_temp_schedule.clone()
                                                        );
                                                    }
                                                }

                                                for (next_temp_temp_schedule) in list_next_schedule {
                                                    let list_violation_temp_schedule =
                                                        self.rule.list_number_constraint_violation(
                                                            &list_constraint_upper_priority,
                                                            &next_temp_temp_schedule
                                                        );
                                                    if !self.rule.is_make_upper_constraint_worse(&list_violation, &list_violation_temp_schedule) {
                                                        next_temp_schedule = next_temp_temp_schedule.clone();
                                                        break;
                                                    }
                                                }
                                            }

                                            if counting_duration_day[&4] > 1 {
                                                let mut num = 0;
                                                while (num < &counting_duration_day[&4] - 1) {
                                                    for day in 0..=6 {
                                                        if solution::get_value(&next_temp_schedule, &staff_, date::convert_to_solution_hashmap_index(&day, &week))
                                                            .unwrap()
                                                            .clone() == "M3".to_string()
                                                        {
                                                            if let Some(inner_map) = next_temp_schedule.get_mut(&staff_) {
                                                                inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), "M1".to_string());
                                                            }
                                                            num += 1;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }

                                        } else {
                                            for day in 0..=6 {
                                                if solution::get_value(
                                                    &next_temp_schedule,
                                                    &staff_,
                                                    date::convert_to_solution_hashmap_index(&day, &week)).unwrap() == "M3"
                                                {
                                                    if let Some(inner_map) = next_temp_schedule.get_mut(&staff_) {
                                                        inner_map.insert(
                                                            date::convert_to_solution_hashmap_index(&day, &week),
                                                            "M2".to_string(),
                                                        );
                                                    };
                                                }
                                            }

                                            let mut counting_duration_day: HashMap<i8, i8> = HashMap::new();
                                            for index in vec![0, 4, 7, 8] {
                                                counting_duration_day.insert(index, 0);
                                            }

                                            for day in 0..=6 {
                                                let current_shift = solution::get_value(
                                                    &next_temp_schedule,
                                                    &staff_,
                                                    date::convert_to_solution_hashmap_index(&day, &week)
                                                );

                                                let current_shift_duration = &self.input.shifts
                                                    .iter()
                                                    .find(|&x| x.id.as_str() == &current_shift.clone().unwrap())
                                                    .unwrap()
                                                    .duration;

                                                counting_duration_day.insert(*current_shift_duration, counting_duration_day.get(current_shift_duration).unwrap() + 1);
                                            }

                                            if counting_duration_day[&0] > 1 {
                                                let mut num = 0;
                                                while (num < counting_duration_day[&4] - 1) {
                                                    for day in 0..=6 {
                                                        if solution::get_value(&next_temp_schedule, &staff_, day).unwrap() == "DO" {
                                                            if let Some(inner_map) = next_temp_schedule.get_mut(&staff_) {
                                                                inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), "M1".to_string());
                                                            }
                                                            num += 1;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }

                                            if counting_duration_day[&8] < 2 {
                                                let mut num = 0;
                                                while (num < 2 - counting_duration_day[&8]) {
                                                    for day in 0..=6 {
                                                        if ["A2", "M2"].contains(&solution::get_value(&next_temp_schedule, &staff_, day).unwrap().as_str()) {
                                                            let new_s = match solution::get_value(&next_temp_schedule, &staff_, day).unwrap().as_str() {
                                                                "A2" => { "A1" }
                                                                "M2" => { "M1" }
                                                                _ => { random::random_choice(&vec!["A1", "M1"]) }
                                                            };
                                                            if let Some(inner_map) = next_temp_schedule.get_mut(&staff_) {
                                                                inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), new_s.parse().unwrap());
                                                            }
                                                            num += 1;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }

                                            if counting_duration_day[&8] > 2 {
                                                let mut num = 0;
                                                while (num < counting_duration_day[&8] - 2) {
                                                    for day in 0..=6 {
                                                        if ["A1", "M1"].contains(&solution::get_value(&next_temp_schedule, &staff_, day).unwrap().as_str()) {
                                                            let new_s = match solution::get_value(&next_temp_schedule, &staff_, day).unwrap().as_str() {
                                                                "A1" => { "A2" }
                                                                "M1" => { "M2" }
                                                                _ => { random::random_choice(&vec!["A2", "M2"]) }
                                                            };
                                                            if let Some(inner_map) = next_temp_schedule.get_mut(&staff_) {
                                                                inner_map.insert(date::convert_to_solution_hashmap_index(&day, &week), new_s.parse().unwrap());
                                                            }
                                                            num += 1;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        "archive-0.5-day" => {
                            for week in 1..= *&self.input.schedule_period {
                                let map_temp_violation = self.rule.constraint_violation(
                                    &constraint,
                                    &week,
                                    &next_temp_schedule
                                );

                                for (staff_, violation) in map_temp_violation{
                                    if violation != 5.5 {
                                        for day in 0..=6 {
                                            let next_temp_temp_schedule = next_temp_schedule.clone();
                                            if ["M2", "A2"].contains(&solution::get_value(
                                                &next_temp_temp_schedule,
                                                &staff_,
                                                date::convert_to_solution_hashmap_index(&day, &week))
                                                .unwrap()
                                                .as_str()
                                            ){
                                                if let Some(inner_map)  = next_temp_schedule.get_mut(&staff_) {
                                                    inner_map.insert(
                                                        date::convert_to_solution_hashmap_index(&day, &week),
                                                        match solution::get_value(
                                                            &next_temp_temp_schedule,
                                                            &staff_,
                                                            date::convert_to_solution_hashmap_index(&day, &week)).unwrap().as_str()
                                                        {
                                                            "M2" => {"M1".to_string()}
                                                            "A2" => {"A1".to_string()}
                                                            _ => {
                                                                random::random_choice(&vec!["M1", "A1"]).to_string()
                                                            }
                                                        }
                                                    );
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        "un-archive-0.5-day" => {
                            for week in 1..= *&self.input.schedule_period {
                                let map_temp_violation = self.rule.constraint_violation(
                                    &constraint,
                                    &week,
                                    &next_temp_schedule
                                );

                                for (staff_, violation) in map_temp_violation{
                                    if violation != 6.0 {

                                        for day in 0..=6 {
                                            let next_temp_temp_schedule = next_temp_schedule.clone();
                                            if solution::get_value(
                                                &next_temp_temp_schedule,
                                                &staff_,
                                                date::convert_to_solution_hashmap_index(&day, &week))
                                                .unwrap().as_str() == "M3"
                                            {
                                                if let Some(inner_map)  = next_temp_schedule.get_mut(&staff_) {
                                                    inner_map.insert(
                                                        date::convert_to_solution_hashmap_index(&day, &week),
                                                        match solution::get_value(
                                                            &next_temp_temp_schedule,
                                                            &staff_,
                                                            date::convert_to_solution_hashmap_index(&day, &week)).unwrap().as_str()
                                                        {
                                                            "M3" => {"M2".to_string()}
                                                            _ => {
                                                                random::random_choice(&vec!["M2", "A2"]).to_string()
                                                            }
                                                        }
                                                    );
                                                };
                                            }
                                        }

                                    }
                                }
                            }
                        }
                        _=> {}
                    };
                }
            }
        }

        next_temp_schedule
    }

    fn adjustment(){

    }

    fn simulated_annealing(
        &mut self,
        schedule: &HashMap<String, HashMap<i8, String>>,
        next_schedule: &HashMap<String,HashMap<i8, String>>
    ) -> HashMap<String, HashMap<i8, String>>
    {
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

    fn shake_and_repair(
        &self,
        schedule: &mut HashMap<String,HashMap<i8, String>>, operator_index: i8
    ) -> HashMap<String, HashMap<i8, String>> {
        let result = match operator_index{
            0 => self.random_swap_staff_shift(schedule),
            1 => self.greedy_coverage_enhancement(schedule),
            2 => self.greedy_horizontal_coverage_enhancement(schedule),
            3 => self.greedy_swap_staff_shift_enhancement(schedule),
            4 => self.greedy_fix_constraint_violation(schedule),
            _ => schedule.clone()
        };

        result
    }

    pub fn print_solution(&self){
        to_excel::write_hashmap_to_excel(&self.solution.clone(),"src/output/output.xlsx");
        let score_coverage = self.score.calculate_coverage_score(&self.solution);
        let h_score_coverage = self.score.calculate_horizontal_coverage_score(&self.solution);
        let score_constraint = self.score.calculate_constraint_score(&self.solution);
        let score_pattern_constraint = self.score.calculate_pattern_constraint_score(&self.solution);
        println!("[coverage score]: {}", score_coverage);
        println!("[horizontal coverage score]: {}", h_score_coverage);
        println!("[constraint score]: {}", score_constraint);
        println!("[pattern-constraint score]: {}", score_pattern_constraint);
    }

    pub fn run_iteration(&mut self){
        let mut current_solution = self.initial_solution();
        self.solution = current_solution.clone();

        for iter_num in 1..= self.max_iteration{
            println!("iter: {}", iter_num);

            let operator_index = self.route_wheel(iter_num);
            self.operator_time[operator_index as usize] += 1.0;

            let next_solution = self.shake_and_repair(&mut current_solution, operator_index);
            current_solution = self.simulated_annealing(&current_solution, &next_solution).clone();

            if (self.score.calculate_total_score(&current_solution) > self.score.calculate_total_score(&self.solution)){
                self.solution = current_solution.clone();
            }
        }

        self.print_solution();
    }
}