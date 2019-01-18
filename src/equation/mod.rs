mod test;
use std::collections::BTreeMap;
use crate::token;
use crate::maths;
use crate::error;

/*
** PARSING ********************************************************************
*/

fn add_coeff_in_table(token: &token::Token, vec_coeff: &mut Vec<i32>) {
    if token::is_plus(&token) {
        vec_coeff.push(1);
    } else if token::is_minus(&token) {
        vec_coeff.push(-1);
    }
}

fn get_coeff_table(tokens: &[token::Token]) -> Vec<i32> {
    let mut vec_coeff: Vec<i32> = Vec::new();
    vec_coeff.push(1);  // first member
    tokens.iter().for_each(|token| add_coeff_in_table(token, &mut vec_coeff));
    return vec_coeff;
}

fn get_exponent(member: &[token::Token]) -> u32 {
    if token::is_coefficient(&member[0]) {
        match member.len() {
            1 => return 0,
            _ => return member[2].exponent
        }
    }
    member[0].exponent
}

fn parse_f64(word: &str) -> f64 {
    match word.parse::<f64>() {
        Err(_) => 0.0,
        Ok(v) => v
    }
}

fn get_coeff(member: &[token::Token]) -> f64 {
    if token::is_indeterminate(&member[0]) {
        match member.len() {
            1 => return 1.0,
            _ => return parse_f64(&member[2].word)
        }
    }
    parse_f64(&member[0].word)
}

/*
** member :
**      a factor.
**      ex : X * 2 or X or 2
** components :
**      an hasmap.
**      - key : exponent
**      - value : coefficient
**      ex: X^3 * -13 -> key = 3 and value = -13
** side_coeff :
**      1 if left side of equation, -1 otherwise.
** coeff_tab :
**      coefficients for each equation factor before spliting (1 or -1).
**      ex : "X * 2 - X + 8" will give [1, -1, 1]
*/

fn parse_each_factor(member: &[token::Token], pos: usize, components: &mut BTreeMap<u32, f64>, side_coeff: i32, coeff_tab: &[i32]) {
    let final_coeff: f64 = (coeff_tab[pos] as f64) * (side_coeff as f64);
    let expo = get_exponent(member);
    match components.contains_key(&expo) {
        false => components.insert(get_exponent(member), final_coeff * get_coeff(member)),
        true => components.insert(get_exponent(member), components[&expo] + final_coeff * get_coeff(member))
    };
}

fn parse_sub_eq(tokens: &[token::Token], components: &mut BTreeMap<u32, f64>, side_coeff: i32) {
    let coeff_tab: Vec<i32> = get_coeff_table(tokens);
    let split = tokens.split(|t| token::is_separator_op(&t));
    split.enumerate().for_each(|(pos, member)| parse_each_factor(member, pos, components, side_coeff, &coeff_tab));
}

pub fn parse(tokens: &Vec<token::Token>) -> BTreeMap<u32, f64> {
    let mut components: BTreeMap<u32, f64> = BTreeMap::new();
    let mut split = tokens.split(|t| token::is_equal(&t));
    let (left, right) = (split.next().unwrap(), split.next().unwrap()); //TODO: Changer le unwrap
    parse_sub_eq(left, &mut components, 1);
    parse_sub_eq(right, &mut components, -1);
    components
}

/*
** DISPLAY ********************************************************************
*/

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

/*
** SOLVER *********************************************************************
*/

fn get_polynomial_degree(components: &BTreeMap<u32, f64>) -> u32 {
    // remove all component with a coeff == 0, then select the exponent max.
    match components.iter().filter(|(_, coeff)| **coeff != 0.0).max_by_key(|(expo, _)| *expo) {
        None => 0,
        Some((expo, _)) => *expo
    }
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
    println!("There is an unique solution.\nSolution: {}", -1.00 * b / a);
    Ok(())
}

fn display_solution_degree2(a: f64, b: f64, c: f64) {
    let delta: f64 = b * b - 4.0 * a * c;
    if delta == 0.0 {
        println!("Discriminant = delta = b^2 - 4ac = 0");
        println!("There is an unique solution.");
        println!("Solution = b^2 / 2a = {}", (-1.0 * b) / (2.0 * a));
    } else if delta > 0.0 {
        println!("Discriminant = delta = b^2 - 4ac = {} > 0", delta);
        println!("There is two solutions.");
        println!("Solution 1 = (-b + sqrt(delta)) / 2a = {}", 1.0);
        println!("Solution 2 = (-b - sqrt(delta)) / 2a = {}", 1.0);
    } else {
        println!("Discriminant = delta = b^2 - 4ac = {} < 0", delta);
        println!("There is two complexe solutions.");
        println!("Solution 1: {} * i", 2.0);
        println!("Solution 2: {} * i", 2.0);
    }
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
    display_solution_degree2(a, b, c);
    Ok(())
}

pub fn solve(components: &BTreeMap<u32, f64>) -> Result<(), error::AppError> {
    let degree = get_polynomial_degree(components);
    println!("Polynomial degree: {}", degree);
    match degree {
        0 => { println!("There is no solution."); Ok(()) },
        1 => solve_degree_1_equation(components),
        2 => solve_degree_2_equation(components),
        _ => Err(error::too_hight_polynomial_degree())
    }
}
