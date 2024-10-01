use std::collections::HashMap;
use std::collections::HashSet;
use xlsxwriter::{Workbook, Format};

pub(crate) fn write_hashmap_to_excel(data: &HashMap<String, HashMap<i8, String>>, file_path: &str) {
    let workbook = Workbook::new(file_path).expect("Failed to create Excel file");
    let mut worksheet = workbook.add_worksheet(None).expect("Failed to create worksheet");
    let header_format = workbook.add_format().set_bold().clone();

    worksheet.write_string(0, 0, "Staff", Some(&header_format)).unwrap();

    let mut unique_days: HashSet<i8> = data
        .values()
        .flat_map(|inner_map| inner_map.keys().copied())
        .collect();

    let mut unique_days: Vec<i8> = unique_days.into_iter().collect();
    unique_days.sort_unstable();

    for (col, day) in unique_days.iter().enumerate() {
        worksheet
            .write_number(0, (col + 1) as u16, *day as f64, Some(&header_format))
            .unwrap();
    }

    let mut sorted_staff_names: Vec<&String> = data.keys().collect();
    sorted_staff_names.sort(); // Sắp xếp staff theo thứ tự alphabet

    let mut row = 1;
    for staff_name in sorted_staff_names {
        worksheet.write_string(row, 0, staff_name, None).unwrap();

        if let Some(shifts_by_day) = data.get(staff_name) {
            for (col, day) in unique_days.iter().enumerate() {
                let shift = shifts_by_day.get(day).unwrap_or(&String::new()).clone();
                worksheet.write_string(row, (col + 1) as u16, &shift, None).unwrap();
            }
        }

        row += 1;
    }

    workbook.close().unwrap();
}