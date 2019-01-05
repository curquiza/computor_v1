use crate::token;


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

#[test]
fn test_get_token_role() {
    assert_eq!(get_token_role("+".to_string()),                     token::Type::SeparationOp);
    assert_eq!(get_token_role("++".to_string()),                    token::Type::Unknown);
    assert_eq!(get_token_role("+-".to_string()),                    token::Type::Unknown);
    assert_eq!(get_token_role("+-".to_string()),                    token::Type::Unknown);

    assert_eq!(get_token_role("*".to_string()),                     token::Type::FactorOp);
    assert_eq!(get_token_role("**".to_string()),                    token::Type::Unknown);

    assert_eq!(get_token_role("3".to_string()),                     token::Type::Coefficient);
    assert_eq!(get_token_role("-3".to_string()),                    token::Type::Coefficient);
    assert_eq!(get_token_role("+3".to_string()),                    token::Type::Coefficient);
    assert_eq!(get_token_role("3u".to_string()),                    token::Type::Unknown);
    assert_eq!(get_token_role("3.".to_string()),                    token::Type::Coefficient);
    assert_eq!(get_token_role("13.2".to_string()),                  token::Type::Coefficient);
    assert_eq!(get_token_role("-1344.2444484948484".to_string()),   token::Type::Coefficient);
    assert_eq!(get_token_role("44.24dd4444".to_string()),           token::Type::Unknown);
    assert_eq!(get_token_role(".254".to_string()),                  token::Type::Coefficient);
    assert_eq!(get_token_role("-.254".to_string()),                 token::Type::Coefficient);
    assert_eq!(get_token_role("+.254+".to_string()),                token::Type::Unknown);

    assert_eq!(get_token_role("X".to_string()),                     token::Type::Indeterminate);
    assert_eq!(get_token_role("X^1".to_string()),                   token::Type::Indeterminate);
    assert_eq!(get_token_role("X^2".to_string()),                   token::Type::Indeterminate);
    assert_eq!(get_token_role("X^-1".to_string()),                  token::Type::Indeterminate);
    assert_eq!(get_token_role("X^a".to_string()),                   token::Type::Indeterminate);
    assert_eq!(get_token_role("X^a1".to_string()),                  token::Type::Indeterminate);
    assert_eq!(get_token_role("XX^3".to_string()),                  token::Type::Unknown);
    assert_eq!(get_token_role("-X^3".to_string()),                  token::Type::Unknown);

    assert_eq!(get_token_role("/".to_string()),                     token::Type::Unknown);
    assert_eq!(get_token_role("%".to_string()),                     token::Type::Unknown);
}

fn check_unknown_token(token: &token::Token) -> Result<(), String> {
    if token::is_unknown(&token) {
        Err(format!("Error: {}: Unexpected token", token.word))
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
        Err(_) => Err(format!("Error: {}: Invalid exponent", word)),
        Ok(value) => Ok(value),
    }
}

#[test]
fn test_get_exponent() {
    assert_eq!(get_exponent(&"X".to_string()), Ok(1));
    assert_eq!(get_exponent(&"X^1".to_string()), Ok(1));
    assert_eq!(get_exponent(&"X^0".to_string()), Ok(0));
    assert_eq!(get_exponent(&"X^+1".to_string()), Ok(1));
    assert_eq!(get_exponent(&"X^13".to_string()), Ok(13));
    assert_eq!(get_exponent(&"X^1352882".to_string()), Ok(1352882));

    assert!(get_exponent(&"X^".to_string()).is_err(), true);
    assert!(get_exponent(&"X^^".to_string()).is_err(), true);
    assert!(get_exponent(&"X^X".to_string()).is_err(), true);
    assert!(get_exponent(&"X^-1".to_string()).is_err(), true);
    assert!(get_exponent(&"X^++1".to_string()).is_err(), true);
    assert!(get_exponent(&"X^a1".to_string()).is_err(), true);
    assert!(get_exponent(&"X^2.5".to_string()).is_err(), true);
    assert!(get_exponent(&"X^-2.5".to_string()).is_err(), true);
    assert!(get_exponent(&"X^1/5".to_string()).is_err(), true);
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
