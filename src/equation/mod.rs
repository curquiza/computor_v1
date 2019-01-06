use crate::token;

pub fn get_degree(tokens: Vec<token::Token>) -> Result<u32, String> {
    let degree: u32 = match tokens.iter().max_by_key(|x| x.exponent) {
        None => return Err("Error while getting equation degree".to_string()),
        Some(v) => v.exponent,
    };
    println!("Equation degree: {}", degree);
    if degree > 2 {
        return Err("Error: degree should be less or equal to 2".to_string());
    }
    Ok(degree)
}
