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

fn main() {
    let split = "Hello les enfants \n titi !".split_whitespace();
    let words_vec: Vec<&str> = split.collect();
    let mut tokens: Vec<Token> = Vec::new();

    for w in words_vec {
        let r: TokenType = TokenType::Unknown;
        let token = Token {word: w.to_string(), role: r};
        tokens.push(token);
    }
    display_all_tokens(tokens);
}
