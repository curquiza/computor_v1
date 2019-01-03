use std::process;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Unknown,
    SeparationOp,
    FactorOp,
    Indeterminate,
    Coefficient
}

struct Token {
    word: String,
    role: TokenType
}

fn role_to_str(role : &TokenType) -> &str {
    match *role {
        TokenType::Unknown => "Unknown",
        TokenType::SeparationOp => "SeparationOp",
        TokenType::FactorOp => "FactorOp",
        TokenType::Indeterminate => "Indeterminate",
        TokenType::Coefficient => "Coefficient",
    }
}

fn token_to_str(token: &Token) -> String {
    let s: String = role_to_str(&token.role).to_owned();
    format!("word = {}, role = {}", token.word, s)
}

// fn display_all_tokens(tokens: Vec<Token>) {
//     for token in tokens {
//         println!("{}", token_to_str(&token));
//     }
// }

fn get_token_role(s: String) -> TokenType {
    if s == "+" || s == "-" {
        TokenType::SeparationOp
    } else if s == "*" {
        TokenType::FactorOp
    } else if s.starts_with("X") {
        if s.len() == 1 {
            TokenType::Indeterminate
        } else if s.starts_with("X^") {
            TokenType::Indeterminate
        } else {
            TokenType::Unknown
        }
    } else if s.parse::<f64>().is_ok() {
        TokenType::Coefficient
    } else {
        TokenType::Unknown
    }
}

#[test]
fn test_get_token_role() {
    assert_eq!(get_token_role("+".to_string()),                     TokenType::SeparationOp);
    assert_eq!(get_token_role("++".to_string()),                    TokenType::Unknown);
    assert_eq!(get_token_role("+-".to_string()),                    TokenType::Unknown);
    assert_eq!(get_token_role("+-".to_string()),                    TokenType::Unknown);

    assert_eq!(get_token_role("*".to_string()),                     TokenType::FactorOp);
    assert_eq!(get_token_role("**".to_string()),                    TokenType::Unknown);

    assert_eq!(get_token_role("3".to_string()),                     TokenType::Coefficient);
    assert_eq!(get_token_role("-3".to_string()),                    TokenType::Coefficient);
    assert_eq!(get_token_role("+3".to_string()),                    TokenType::Coefficient);
    assert_eq!(get_token_role("3u".to_string()),                    TokenType::Unknown);
    assert_eq!(get_token_role("3.".to_string()),                    TokenType::Coefficient);
    assert_eq!(get_token_role("13.2".to_string()),                  TokenType::Coefficient);
    assert_eq!(get_token_role("-1344.2444484948484".to_string()),   TokenType::Coefficient);
    assert_eq!(get_token_role("44.24dd4444".to_string()),           TokenType::Unknown);
    assert_eq!(get_token_role(".254".to_string()),                  TokenType::Coefficient);
    assert_eq!(get_token_role("-.254".to_string()),                 TokenType::Coefficient);
    assert_eq!(get_token_role("+.254+".to_string()),                TokenType::Unknown);

    assert_eq!(get_token_role("X".to_string()),                     TokenType::Indeterminate);
    assert_eq!(get_token_role("X^1".to_string()),                   TokenType::Indeterminate);
    assert_eq!(get_token_role("X^2".to_string()),                   TokenType::Indeterminate);
    assert_eq!(get_token_role("X^-1".to_string()),                  TokenType::Indeterminate);
    assert_eq!(get_token_role("X^a".to_string()),                   TokenType::Indeterminate);
    assert_eq!(get_token_role("X^a1".to_string()),                  TokenType::Indeterminate);
    assert_eq!(get_token_role("XX^3".to_string()),                  TokenType::Unknown);
    assert_eq!(get_token_role("-X^3".to_string()),                  TokenType::Unknown);

    assert_eq!(get_token_role("/".to_string()),                     TokenType::Unknown);
    assert_eq!(get_token_role("%".to_string()),                     TokenType::Unknown);
}

fn check_unknown_token(token: & Token) -> Result<(), String> {
    if token.role == TokenType::Unknown {
        Err(format!("Error: {}: Unexpected token", token.word))
    } else {
        Ok(())
    }
}

fn get_exponent(word: & String) -> Result<u32, String> {
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
    assert_eq!(get_exponent(& "X".to_string()), Ok(1));
    assert_eq!(get_exponent(& "X^1".to_string()), Ok(1));
    assert_eq!(get_exponent(& "X^0".to_string()), Ok(0));
    assert_eq!(get_exponent(& "X^+1".to_string()), Ok(1));
    assert_eq!(get_exponent(& "X^13".to_string()), Ok(13));
    assert_eq!(get_exponent(& "X^1352882".to_string()), Ok(1352882));

    assert!(get_exponent(& "X^".to_string()).is_err(), true);
    assert!(get_exponent(& "X^^".to_string()).is_err(), true);
    assert!(get_exponent(& "X^X".to_string()).is_err(), true);
    assert!(get_exponent(& "X^-1".to_string()).is_err(), true);
    assert!(get_exponent(& "X^++1".to_string()).is_err(), true);
    assert!(get_exponent(& "X^a1".to_string()).is_err(), true);
    assert!(get_exponent(& "X^2.5".to_string()).is_err(), true);
    assert!(get_exponent(& "X^-2.5".to_string()).is_err(), true);
    assert!(get_exponent(& "X^1/5".to_string()).is_err(), true);
}

fn exit_with_error(error: String) {
    eprintln!("{}", error);
    process::exit(1)
}

fn main() {
    // let split = "3 + 2 * X^1 - +4 * X + -2 * X^2".split_whitespace();
    let split = "3 + 2 * X^-1 - +4 * X + -2 * X^2".split_whitespace();
    // let split = "3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2".split_whitespace();
    let words_vec: Vec<&str> = split.collect();
    let mut tokens: Vec<Token> = Vec::new();

    for w in words_vec {
        let r: TokenType = get_token_role(w.to_string());
        let token = Token { word: w.to_string(), role: r };

        println!("{}", token_to_str(& token)); // DEBUG

        if let Err(e) = check_unknown_token(& token) {
            exit_with_error(e)
        }

        if token.role == TokenType::Indeterminate {
            match get_exponent(&token.word) {
                Ok(expo) => println!("exponent = {}", expo), //DEBUG
                Err(e) => exit_with_error(e),
            }
        }

        // si le token = unknow ou token = indeterminate mais avec un mauvaise exponent -> lexical erreur
        tokens.push(token);
    }
    // parsing :
    // coeff = entouré de rien/SeparationOp et rien/FactorOp
    // indeterminate = entouré de rien/SeparationOp et rien/FactorOp
    // factorop = entouré de coeff et indeterminate
    // separatorop = entouré de coeff et indeterminate
    // check exponent du polynome
    // resoudre si tout est ok
    //display_all_tokens(tokens);
}
