mod test;
use crate::token;

pub struct Component {
    pub exponent:   u32,
    pub coeff:      f64
}

pub fn get_degree(tokens: &Vec<token::Token>) -> Result<u32, String> {
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

fn get_equal_pos(tokens: &Vec<token::Token>) -> Result<usize, String> {
    match tokens.iter().position(|t| t.role == token::Type::Equal) {
        None => Err("Error while getting equal position".to_string()),
        Some(v) => Ok(v)
    }
}

// fn get_exponent(pos: usize, tokens: &Vec<token::Token>) -> Result <u32, String> {
//     if pos == 0 || pos == tokens.len() - 1 {
//         return Err("Error while parsing equation".to_string())
//     };
//     if token::is_indeterminate(&tokens[pos - 1]) {
//         return Ok(tokens[pos - 1].exponent);
//     } else if token::is_indeterminate(&tokens[pos + 1]) {
//         return Ok(tokens[pos + 1].exponent);
//     }
//     Err("Error while parsing equation".to_string())
// }
//
// fn parse_sub_vector(tokens: &Vec<token::Token>, gen_coeff: i32, mut components: [Component; 3]) -> Result<(), String>{
//     for (i, token) in tokens.iter().enumerate() {
//         if token::is_factor_op(&token) {
//            let expo = match get_exponent(i, tokens) {
//                 Ok(v) => v,
//                 Err(e) => return Err(e),
//            };
//            // let expo = match get_coeff(i, tokens) {
//            //      Ok(v) => v,
//            //      Err(e) => return Err(e),
//            // };
//         }
//     };
//     components[0].coeff += 1.0;
//     Ok(())
// }

pub fn parse(tokens: &Vec<token::Token>) -> Result<[Component; 3], String> {
    let components: [Component; 3] =
        [
            Component {exponent: 2, coeff: 0.0},
            Component {exponent: 1, coeff: 0.0},
            Component {exponent: 0, coeff: 0.0}
        ];
    let pos = match get_equal_pos(tokens) {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    let (left, right) = tokens.split_at(pos);
    // let titi = &right[1..];
    // token::display_all_slice(right);
    // token::display_all_slice(titi);
    Ok(components)
}
