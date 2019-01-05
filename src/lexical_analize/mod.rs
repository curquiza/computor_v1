use crate::token;
mod test;

fn get_token_role(s: String) -> token::Type {
    if s == "+" || s == "-" {
        token::Type::SeparationOp
    } else if s == "*" {
        token::Type::FactorOp
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

fn check_unknown_token(token: &token::Token) -> Result<(), String> {
    if token::is_unknown(&token) {
        Err(format!("Lexical error: {}: Unexpected token", token.word))
    } else {
        Ok(())
    }
}

fn get_exponent(word: &String) -> Result<u32, String> {
    if word == "X" {
        return Ok(1)
    }
    let substr = &word[2..];
    match substr.parse::<u32>() {
        Err(_) => Err(format!("Lexical error: {}: Invalid exponent", word)),
        Ok(value) => Ok(value),
    }
}

pub fn tokenize(s: String) -> Result<Vec<token::Token>, String> {
    let split = s.split_whitespace();
    let words_vec: Vec<&str> = split.collect();
    let mut tokens: Vec<token::Token> = Vec::new();

    for w in words_vec {
        let r: token::Type = get_token_role(w.to_string());
        let token = token::Token { word: w.to_string(), role: r };

        println!("{}", token::to_str(&token)); // DEBUG

        if let Err(e) = check_unknown_token(&token) {
            return Err(e)
        }

        if token::is_indeterminate(&token) {
            match get_exponent(&token.word) {
                Ok(expo) => println!("exponent = {}", expo), //DEBUG
                Err(e) => return Err(e),
            }
        }

        tokens.push(token);
    }
    Ok(tokens)
}
