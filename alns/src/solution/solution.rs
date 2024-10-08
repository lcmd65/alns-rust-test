use std::collections::HashMap;
use crate::shift::shift::Shift;

pub fn get_value(
    schedule: &HashMap<String, HashMap<i8, String>>,
    staff: &str,
    day: i8
) -> Option<String> {
    schedule.get(staff)
        .and_then(|days_map| days_map.get(&day))
        .cloned()
}

pub fn get_duration(
    schedule: &HashMap<String, HashMap<i8, String>>,
    staff: &str,
    day: i8,
    shift_list: &Vec<Shift>
) -> i8{
    let shift = schedule.get(staff)
        .and_then(|days_map| days_map.get(&day))
        .cloned();

    let duration = shift_list.iter().find(|&x| x.id == shift
        .clone()
        .unwrap()
        .to_string()
    ).unwrap().duration;

    duration
}


pub fn is_violation_core_day(
    schedule: &HashMap<String, HashMap<i8, String>>,
    staff: &String, index: i8
) -> bool{
    if schedule[staff].get(&index).unwrap().as_str() =="PH" ||
        schedule[staff].get(&index).unwrap().as_str() == "DO" {
        return true
    };

    false
}

pub fn is_violation_public_holiday(
    schedule: &HashMap<String, HashMap<i8, String>>,
    staff: &String, index: i8
) -> bool{
    if schedule[staff].get(&index).unwrap().as_str() == "PH" {
        return true;
    }

    false
}

pub fn is_a_shift(
    schedule: &HashMap<String, HashMap<i8, String>>,
    staff: &String, index: i8,
    shift_information: String
) -> bool{
    if schedule[staff].get(&index).unwrap().to_string() == shift_information {
        return true;
    }

    false
}

pub fn is_in_shift_list(
    schedule: &HashMap<String, HashMap<i8, String>>,
    staff: &String,
    index: i8,
    shift_information: Vec<String>
) -> bool{
    for shift in shift_information {
        if schedule[&staff.clone()].get(&index).unwrap().to_string() == shift {
            return true;
        }
    }
    false
}