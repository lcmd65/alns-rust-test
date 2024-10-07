use std::collections::HashMap;
use regex::Regex;

pub struct FormulaExecutor;

impl FormulaExecutor {
    pub fn executor_rust(
        &self,
        formula_string: &str,
        input: &mut HashMap<String, f32>
    ) -> HashMap<String, f32> {
        let values = self.extract_values(formula_string);
        let default_values = values[0];
        let threshold = values[1];
        let step = values[2];

        let mut map: HashMap<String, f32> = HashMap::new();
        for (key, gap) in input.iter() {
            let new_value = (default_values - (gap - threshold).abs() * step).max(0.0);
            map.insert(key.clone(), new_value);
        }
        map
    }

    fn extract_values(&self, input: &str) -> Vec<f32> {
        let regex = Regex::new(r"\b\d+\.\d+|\b\d+").unwrap();
        regex
            .find_iter(input)
            .map(|mat| mat.as_str().parse::<f32>().unwrap())
            .collect()
    }
}