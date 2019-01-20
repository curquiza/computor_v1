use std::collections::BTreeMap;
use crate::maths;
use crate::error;

// public because of tests
pub fn get_polynomial_degree(components: &BTreeMap<u32, f64>) -> u32 {
    // remove all component with a coeff == 0, then select the exponent max.
    match components.iter().filter(|(_, coeff)| **coeff != 0.0).max_by_key(|(expo, _)| *expo) {
        None => 0,
        Some((expo, _)) => *expo
    }
}

fn solve_degree_0_equation(components: &BTreeMap<u32, f64>) -> Result<(), error::AppError> {
    let coeff: f64 = match components.get(&0) {
        None => 0.0,
        Some(v) => *v
    };
    if coeff == 0.0 {
        println!("All real numbers are solution.")
    } else {
        println!("There is no solution.")
    };
    Ok(())
}

fn solve_degree_1_equation(components: &BTreeMap<u32, f64>) -> Result<(), error::AppError> {
    let a: f64 = match components.get(&1) {
        None => return Err(error::when_solving_degree1_eq()),
        Some(v) => *v
    };
    let b: f64 = match components.get(&0) {
        None => 0.0,
        Some(v) => *v
    };
    println!("Coefficients are : {{ a = {}, b = {} }}", a, b);
    println!("There is an unique solution.\nSolution = {}", -1.0 * b / a);
    Ok(())
}

fn display_solution_zero_delta(a: f64, b: f64) {
    println!("Discriminant = delta = b^2 - 4ac = 0");
    println!("There is an unique solution.");
    println!("Solution = b^2 / 2a = {}", (-1.0 * b) / (2.0 * a));
}

fn display_solution_positive_delta(a: f64, b: f64, delta: f64, delta_sqrt: f64) {
    println!("Discriminant = delta = b^2 - 4ac = {} > 0", delta);
    println!("There is two solutions.");
    println!("sqrt(delta) = {}", delta_sqrt);
    println!("Solution 1 = (-b + sqrt(delta)) / 2a = {}", (-1.0 * b + delta_sqrt) / (2.0 * a));
    println!("Solution 2 = (-b - sqrt(delta)) / 2a = {}", (-1.0 * b - delta_sqrt) / (2.0 * a));
}

fn display_solution_negative_delta(a: f64, b: f64, delta: f64, delta_sqrt: f64) {
    println!("Discriminant = delta = b^2 - 4ac = {} < 0", delta);
    println!("There is two complexe solutions.");
    println!("sqrt(|delta|) = {}", delta_sqrt);
    println!("Solution 1 = (-b + sqrt(|delta|) * i) / 2a = -b / 2a + i * (sqrt(|delta|) / 2a) = {} + i * {}", -1.0 * b / (2.0 * a), delta_sqrt / (2.0 * a));
    println!("Solution 2 = (-b - sqrt(|delta|) * i) / 2a = -b / 2a - i * (sqrt(|delta|) / 2a) = {} - i * {}", -1.0 * b / (2.0 * a), delta_sqrt / (2.0 * a));
}

fn display_solution_degree2(a: f64, b: f64, c: f64) -> Result<(), error::AppError> {
    let delta: f64 = b * b - 4.0 * a * c;
    if delta == 0.0 {
        display_solution_zero_delta(a, b);
    } else if delta > 0.0 {
        let delta_sqrt = match maths::sqrt(delta) {
            None => return Err(error::when_solving_degree2_eq()),
            Some(v) => v
        };
        display_solution_positive_delta(a, b, delta, delta_sqrt);
    } else {
        let delta_sqrt = match maths::sqrt(maths::abs(delta)) {
            None => return Err(error::when_solving_degree2_eq()),
            Some(v) => v
        };
        display_solution_negative_delta(a, b, delta, delta_sqrt);
    }
    Ok(())
}

fn solve_degree_2_equation(components: &BTreeMap<u32, f64>) -> Result<(), error::AppError> {
    let a: f64 = match components.get(&2) {
        None => return Err(error::when_solving_degree2_eq()),
        Some(v) => *v
    };
    let b: f64 = match components.get(&1) {
        None => 0.0,
        Some(v) => *v
    };
    let c: f64 = match components.get(&0) {
        None => 0.0,
        Some(v) => *v
    };
    println!("Coefficients are : {{ a = {}, b = {}, c = {} }}", a, b, c);
    if let Err(e) = display_solution_degree2(a, b, c) {
        return Err(e)
    };
    Ok(())
}

pub fn display_rslt(components: &BTreeMap<u32, f64>) -> Result<(), error::AppError> {
    let degree = get_polynomial_degree(components);
    println!("Polynomial degree: {}", degree);
    match degree {
        0 => solve_degree_0_equation(components),
        1 => solve_degree_1_equation(components),
        2 => solve_degree_2_equation(components),
        _ => Err(error::too_high_polynomial_degree())
    }
}
