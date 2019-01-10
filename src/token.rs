#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Unknown,
    SeparationOp,
    FactorOp,
    Indeterminate,
    Coefficient,
    Equal
}

pub struct Token {
    pub word:       String,
    pub role:       Type,
    pub exponent:   u32
}

#[allow(dead_code)]
fn role_to_str(role : &Type) -> &str {
    match *role {
        Type::Unknown => "Unknown",
        Type::SeparationOp => "SeparationOp",
        Type::FactorOp => "FactorOp",
        Type::Indeterminate => "Indeterminate",
        Type::Coefficient => "Coefficient",
        Type::Equal => "Equal",
    }
}

#[allow(dead_code)]
pub fn to_str(token: &Token) -> String {
    let s: String = role_to_str(&token.role).to_owned();
    format!("word = {}, role = {}, exponent = {}", token.word, s, token.exponent)
}

#[allow(dead_code)]
pub fn display_all(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{}", to_str(&token));
    }
}

pub fn is_equal(token: &Token) -> bool {
    token.role == Type::Equal
}
pub fn is_separator_op(token: &Token) -> bool {
    token.role == Type::SeparationOp
}

pub fn is_factor_op(token: &Token) -> bool {
    token.role == Type::FactorOp
}

pub fn is_operator(token: &Token) -> bool {
    token.role == Type::SeparationOp || token.role == Type::FactorOp
}

pub fn is_indeterminate(token: &Token) -> bool {
    token.role == Type::Indeterminate
}

pub fn is_coefficient(token: &Token) -> bool {
    token.role == Type::Coefficient
}

pub fn is_member(token: &Token) -> bool {
    token.role == Type::Indeterminate || token.role == Type::Coefficient
}

pub fn is_unknown(token: &Token) -> bool {
    token.role == Type::Unknown
}
