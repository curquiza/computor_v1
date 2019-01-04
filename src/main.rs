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

// fn display_all_tokens(tokens: &Vec<Token>) {
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

fn check_unknown_token(token: &Token) -> Result<(), String> {
    if is_unknown(&token) {
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

fn exit_with_error(error: String) {
    eprintln!("{}", error);
}

fn syntax_error_msg(word: &String) -> String {
    format!("Syntax error near \"{}\" token", word)
}

fn is_separator_op(token: &Token) -> bool {
    token.role == TokenType::SeparationOp
}

fn is_factor_op(token: &Token) -> bool {
    token.role == TokenType::FactorOp
}

fn is_operator(token: &Token) -> bool {
    token.role == TokenType::SeparationOp || token.role == TokenType::FactorOp
}

fn is_indeterminate(token: &Token) -> bool {
    token.role == TokenType::Indeterminate
}

fn is_coefficient(token: &Token) -> bool {
    token.role == TokenType::Coefficient
}

fn is_member(token: &Token) -> bool {
    token.role == TokenType::Indeterminate || token.role == TokenType::Coefficient
}

fn is_unknown(token: &Token) -> bool {
    token.role == TokenType::Unknown
}

/*
 ** check_operator_syntax :
 ** An operator (+ or *) :
 ** - is not at start or end position
 ** - does not touch any other operator
 ** - if is a FactorOp, the sides can't be the sames
 */

fn check_operator_syntax(pos: usize, tokens: &Vec<Token>) -> Result<(), String> {
    if pos == 0 || pos == tokens.len() - 1                                              // at the end or at start
        || is_operator(&tokens[pos - 1]) || is_operator(&tokens[pos + 1]) {             // sides are operators
        return Err(syntax_error_msg(& tokens[pos].word))
    // } else if is_operator(&tokens[pos - 1]) || is_operator(&tokens[pos + 1]) {
        // return Err(syntax_error_msg(& tokens[pos].word))
    } else if is_factor_op(&tokens[pos]) {                                              // is FactorOp
        if (is_indeterminate(&tokens[pos - 1]) && is_indeterminate(&tokens[pos + 1]))   // but sides are
            || (is_coefficient(&tokens[pos - 1]) && is_coefficient(&tokens[pos + 1])) { // the same
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
fn check_member_syntax(pos: usize, tokens: &Vec<Token>) -> Result<(), String> {
    if pos == 0 {                                                           // at start
        if tokens.len() == 1 || is_operator(&tokens[pos + 1]) {
            return Ok(())
        }
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if pos == tokens.len() - 1 && is_member(&tokens[pos - 1]) {      // at the end
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if is_member(&tokens[pos - 1]) || is_member(&tokens[pos + 1]) {  // sides are not operators
        return Err(syntax_error_msg(& tokens[pos].word))
    } else if tokens[pos - 1].role == tokens[pos + 1].role                  // sides are the same
                && !is_separator_op(&tokens[pos]) {                         // but is not SeparationOp
        return Err(syntax_error_msg(& tokens[pos].word))
    }
    Ok(())
}

fn check_syntax(tokens: &Vec<Token>) -> Result<(), String> {
    for (i, token) in tokens.iter().enumerate() {
        if is_operator(&token) {
            if let Err(e) = check_operator_syntax(i, tokens) {
                return Err(e)
            }
        } else if let Err(e) = check_member_syntax(i, tokens) {
            return Err(e)
        }
    }
    Ok(())
}

fn tokenization(s: String) -> Result<Vec<Token>, String> {
    let split = s.split_whitespace();
    let words_vec: Vec<&str> = split.collect();
    let mut tokens: Vec<Token> = Vec::new();

    for w in words_vec {
        let r: TokenType = get_token_role(w.to_string());
        let token = Token { word: w.to_string(), role: r };

        println!("{}", token_to_str(&token)); // DEBUG

        if let Err(e) = check_unknown_token(&token) {
            return Err(e)
        }

        if is_indeterminate(&token) {
            match get_exponent(&token.word) {
                Ok(expo) => println!("exponent = {}", expo), //DEBUG
                Err(e) => return Err(e),
            }
        }

        tokens.push(token);
    }
    Ok(tokens)
}

fn main() {
    let s = "3 + 2 * X^1 - +4 * X + -2 * X^2".to_string();
    // let s = "3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2".to_string();
    let tokens = match tokenization(s) {
        Ok(t) => t,
        Err(e) => return exit_with_error(e),
    };
    if let Err(e) = check_syntax(&tokens) {
        return exit_with_error(e);
    }
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
