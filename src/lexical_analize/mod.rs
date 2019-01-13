use crate::token;
mod test;

fn get_token_role(s: &str) -> token::Type {
    if s == "+" || s == "-" {
        token::Type::SeparationOp
    } else if s == "*" {
        token::Type::FactorOp
    } else if s == "=" {
        token::Type::Equal
    } else if s.starts_with("X") {
        if s.len() == 1 {
            token::Type::Indeterminate
        } else if s.starts_with("X^") {
            token::Type::Indeterminate
        } else {
            token::Type::Unknown
        }
    } else if s.parse::<f64>().is_ok() {
        token::Type::Coefficient
    } else {
        token::Type::Unknown
    }
}

fn check_unknown_token(token: &token::Token) -> Result<(), &str> {
    if token::is_unknown(token) {
        Err("toto")
        // Err(&format!("Lexical error: {}: Unexpected token", token.word))
    } else {
        Ok(())
    }
}

fn check_exponent(token: &token::Token) -> Result<u32, &str> {
    if token.word == "X" {
        return Ok(1)
    }
    if token.word.len() < 2 {
        return Err("Error when checking exponent in lexical analize")
    }
    let substr = &token.word[2..];
    match substr.parse::<u32>() {
        // Err(_) => Err(format!("Lexical error: {}: Invalid exponent", token.word)),
        Err(_) => Err("Lexical error: Invalid exponent"),
        Ok(v) => Ok(v),
    }
}

fn handle_lexical_errors(token: &mut token::Token) -> Result<(), &str> {
    if let Err(e) = check_unknown_token(&token) {
        return Err(e)
    }
    // if token::is_indeterminate(&token) {
    //     match check_exponent(&token) {
    //         Err(e) => return Err(e),
    //         Ok(v) => {
    //             token.exponent = v;
    //             return Ok(())
    //         }
    //     }
    // }
    Ok(())
}

pub fn tokenize(s: &str) -> Result<Vec<token::Token>, &str> {
    let split = s.split_whitespace();
    let words_vec: Vec<&str> = split.collect();
    if words_vec.len() == 0 {
        return Err("Empty argument")
    }

    let mut tokens: Vec<token::Token> = Vec::new();
    for w in words_vec {
        let r: token::Type = get_token_role(w);
        let mut token = token::Token { word: w.to_string(), role: r, exponent: 0 };
        if let Err(e) = handle_lexical_errors(&mut token) {
            return Err(e);
        }
        println!("{}", token); //DEBUG
        tokens.push(token);
    }

    Ok(tokens)
}
