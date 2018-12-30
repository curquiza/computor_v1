#[derive(PartialEq, Eq)]
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

fn display_all_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{}", token_to_str(&token));
    }
}

fn get_token_role(s: String) -> TokenType {
    if s == "+" || s == "-" {
        TokenType::SeparationOp
    } else if s == "*" {
        TokenType::FactorOp
    } else if s.starts_with("X") {
        if s.len() == 1 {
            TokenType::Indeterminate
        } else if s.starts_with("X^") {
            //let substr = &s[2..];
            //if substr.parse::<u32>().is_err() {
                //TokenType::Unknown
            //} else {
                //TokenType::Indeterminate
            //}
            TokenType::Indeterminate
        } else {
            TokenType::Unknown
        }
    } else if s.parse::<f32>().is_ok() {
        TokenType::Coefficient
    } else {
        TokenType::Unknown
    }
}

fn main() {
    //let split = "3 + 2 * X^1 - +4 * X + -2 * X^2".split_whitespace();
    let split = "3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2".split_whitespace();
    let words_vec: Vec<&str> = split.collect();
    let mut tokens: Vec<Token> = Vec::new();

    for w in words_vec {
        //let r: TokenType = TokenType::Unknown;
        let r: TokenType = get_token_role(w.to_string());
        let token = Token {word: w.to_string(), role: r};
        println!("{}", token_to_str(&token));
        // si le token = unknow ou token = indeterminate mais avec un mauvaise exposant -> lexical erreur
        tokens.push(token);
    }
    // parsing :
    // coeff = entouré de rien/SeparationOp et rien/FactorOp
    // indeterminate = entouré de rien/SeparationOp et rien/FactorOp
    // factorop = entouré de coeff et indeterminate
    // separatorop = entouré de coeff et indeterminate
    // check exposant du polynome
    // resoudre si tout est ok
    //display_all_tokens(tokens);
}
