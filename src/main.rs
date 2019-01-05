mod lexical_analize;
mod token;
// use std::mem;

fn syntax_error_msg(word: &String) -> String {
    format!("Syntax error near \"{}\" token", word)
}

/*
 ** check_operator_syntax :
 ** An operator (+ or *) :
 ** - is not at start or end position
 ** - does not touch any other operator
 ** - if is a FactorOp, the sides can't be the sames
 */

fn check_operator_syntax(pos: usize, tokens: &Vec<token::Token>) -> Result<(), String> {
    if pos == 0 || pos == tokens.len() - 1                                              // at the end or at start
        || token::is_operator(&tokens[pos - 1]) || token::is_operator(&tokens[pos + 1]) {             // sides are operators
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if token::is_factor_op(&tokens[pos]) {                                              // is FactorOp
        if (token::is_indeterminate(&tokens[pos - 1]) && token::is_indeterminate(&tokens[pos + 1]))   // but sides are
            || (token::is_coefficient(&tokens[pos - 1]) && token::is_coefficient(&tokens[pos + 1])) { // the same
                return Err(syntax_error_msg(& tokens[pos].word))
            }
    }
    Ok(())
}

/*
 ** check_member_syntax :
 ** A member (coefficient or X) ;
 ** - can be at start or end position
 ** - does not touch any other member
 ** - if both sides are the same operators, it must be a SeparationOp (+)
 */
fn check_member_syntax(pos: usize, tokens: &Vec<token::Token>) -> Result<(), String> {
    if pos == 0 {                                                           // at start
        if tokens.len() == 1 || token::is_operator(&tokens[pos + 1]) {
            return Ok(())
        }
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if pos == tokens.len() - 1 && token::is_member(&tokens[pos - 1]) {      // at the end
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if token::is_member(&tokens[pos - 1]) || token::is_member(&tokens[pos + 1]) {  // sides are not operators
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if tokens[pos - 1].role == tokens[pos + 1].role                  // sides are the same
                && !token::is_separator_op(&tokens[pos]) {                         // but is not SeparationOp
        return Err(syntax_error_msg(& tokens[pos].word))
    }
    Ok(())
}

fn check_syntax(tokens: &Vec<token::Token>) -> Result<(), String> {
    for (i, token) in tokens.iter().enumerate() {
        if token::is_operator(&token) {
            if let Err(e) = check_operator_syntax(i, tokens) {
                return Err(e)
            }
        } else if let Err(e) = check_member_syntax(i, tokens) {
            return Err(e)
        }
    }
    Ok(())
}

fn exit_with_error(error: String) -> Result<(), Box<std::error::Error>> {
    eprintln!("{}", error);
    Err(error.into())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let s = "3 + 2 * X^1 - +4 * X + -2 * X^2".to_string();
    // let s = "3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2".to_string();

    let tokens = match lexical_analize::tokenize(s) {
        Ok(t) => t,
        Err(e) => return exit_with_error(e),
    };
    if let Err(e) = check_syntax(&tokens) {
        return exit_with_error(e);
    }
    Ok(())
    // TODO: check exponent du polynome
    // Exemple tests check syntax
    // X * 4 + 4 * X + 5 + 6 * X + X + 3 * X
    // X * X + 4 * X
    // 4 * X * 2 + 2
    // 4 *
    // +
    // TODO: parsing
    // TODO: resoudre
}
