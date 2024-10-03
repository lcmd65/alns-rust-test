

pub fn is_leap_year()->bool{
    false
}

pub fn convert_to_solution_hashmap_index(
    day: &i8,
    week: &i8
) -> i8{

    day + 7 * (week -1)
}