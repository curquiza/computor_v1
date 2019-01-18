mod test;
use std::collections::HashMap;
use crate::token;

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
    if token::is_factor_op(&member[0]) {
        match member.len() {
            1 => return 0,
            _ => return member[2].exponent
        }
    }
    member[0].exponent
}

fn parse_int32(word: &str) -> i32 {
    match word.parse::<i32>() {
        Err(_) => 0,
        Ok(v) => v
    }
}

fn get_coeff(member: &[token::Token]) -> i32 {
    if token::is_indeterminate(&member[0]) {
        match member.len() {
            1 => return 1,
            _ => return parse_int32(&member[2].word)
        }
    }
    parse_int32(&member[0].word)
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

fn parse_each_factor(member: &[token::Token], pos: usize, components: &mut HashMap<u32, i32>, side_coeff: i32, coeff_tab: &[i32]) {
    let expo = get_exponent(member);
    match components.contains_key(&expo) {
        false => components.insert(get_exponent(member), coeff_tab[pos] * side_coeff * get_coeff(member)),
        true => components.insert(get_exponent(member), components[&expo] + coeff_tab[pos] * side_coeff * get_coeff(member))
    };
}

fn parse_sub_eq(tokens: &[token::Token], components: &mut HashMap<u32, i32>, side_coeff: i32) {
    let coeff_tab: Vec<i32> = get_coeff_table(tokens);
    let split = tokens.split(|t| token::is_separator_op(&t));
    split.enumerate().for_each(|(pos, member)| parse_each_factor(member, pos, components, side_coeff, &coeff_tab));
}

pub fn parse(tokens: &Vec<token::Token>) -> HashMap<u32, i32> {
    let mut components: HashMap<u32, i32> = HashMap::new();
    let mut split = tokens.split(|t| token::is_equal(&t));
    let (left, right) = (split.next().unwrap(), split.next().unwrap()); //TODO: Changer le unwrap
    parse_sub_eq(left, &mut components, 1);
    parse_sub_eq(right, &mut components, -1);
    components
}

/*
** DISPLAY ********************************************************************
*/

/*
** SOLVER *********************************************************************
*/

