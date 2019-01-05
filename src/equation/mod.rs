use crate::token;
// use std::str::FromStr;

fn get_exponent(token: &token::Token) -> Result<u32, String> {
    if token.word == "X" {
        return Ok(1)
    }
    let substr = &token.word[2..];
    match substr.parse::<u32>() {
        Err(_) => return Err("Error while getting equation degree".to_string()),
        Ok(v) => return Ok(v),
    }
}

pub fn get_degree(tokens: Vec<token::Token>) -> Result<u32, String> {
    // let mut degree = 0;
    //
    // for token in tokens {
    //     if token::is_indeterminate(&token) {
    //         tmp = get_exponent(&token.word);
    //         degree = get_exponent(&token.word) 
    //     }
    // }
    // tokens.iter().max_by(|x, y| )
    let degree = 0;
    println!("Equation degree: {}", degree);
    if degree <= 2 {
        return Err("Error: degree should be less or equal to 2".to_string());
    }
    Ok(degree)
}
