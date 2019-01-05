use crate::token;

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
    println!("{} is operator", tokens[pos].word);
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
    println!("{} is member", tokens[pos].word);
    if pos == 0 {                                                           // at start
        if tokens.len() == 1 || token::is_operator(&tokens[pos + 1]) {
            return Ok(())
        }
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if pos == tokens.len() - 1 {
        if token::is_member(&tokens[pos - 1]) {      // at the end
            return Err(syntax_error_msg(& tokens[pos].word))
        }
        return Ok(())
    } else if token::is_member(&tokens[pos - 1]) || token::is_member(&tokens[pos + 1]) {  // sides are not operators
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if tokens[pos - 1].role == tokens[pos + 1].role                  // sides are the same
                && !token::is_separator_op(&tokens[pos]) {                         // but is not SeparationOp
        return Err(syntax_error_msg(& tokens[pos].word))
    }
    Ok(())
}

pub fn check_syntax(tokens: &Vec<token::Token>) -> Result<(), String> {
    for (i, token) in tokens.iter().enumerate() {
        println!("i = {} !!!", i);
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
