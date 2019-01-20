pub mod parsing;
pub mod solver;
mod test;

use std::collections::BTreeMap;

fn add_one_factor_to_eq(expo: u32, coeff: f64, s: &mut String) {
    if coeff != 0.0 {
        if coeff == 1.0 { // impossible to use float in pattern matching
            match expo {
                0 => s.push_str(" 1 +"),
                1 => s.push_str(" X +"),
                e => s.push_str(&format!(" X^{} +", e))
            }
        } else {
            match (coeff, expo) {
                (c, 0) => s.push_str(&format!(" {} +", c)),
                (c, 1) => s.push_str(&format!(" {} * X +", c)),
                (c, e) => s.push_str(&format!(" {} * X^{} +", c, e)),
            }
        }
    }
}

fn get_reduced_form(components: &BTreeMap<u32, f64>) -> String {
    let mut equation: String = String::new();
    components.iter().for_each(|(expo, coeff)| add_one_factor_to_eq(*expo, *coeff, &mut equation));
    if equation.len() < 2 {
        return "0".to_string();
    }
    let end = equation.len() - 2;
    equation[1..end].to_string()
}

pub fn display_reduced_eq(components: &BTreeMap<u32, f64>) {
    println!("Reduced form: {} = 0", get_reduced_form(&components));
}
