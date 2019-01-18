#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Unknown,
    SeparationOp,
    FactorOp,
    Indeterminate,
    Coefficient,
}

pub struct Token {
    pub word:       String,
    pub role:       Type,
    pub exponent:   u32,
}

use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unknown => write!(f, "Unknown"),
            Type::SeparationOp => write!(f, "SeparationOp"),
            Type::FactorOp => write!(f, "FactorOp"),
            Type::Indeterminate => write!(f, "Indeterminate"),
            Type::Coefficient => write!(f, "Coefficient"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(word = {}, role = {}, exponent = {})", self.word, self.role, self.exponent)
    }
}

#[allow(dead_code)]
pub fn display_all(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{}", token);
    }
}

#[allow(dead_code)]
pub fn display_all_slice(tokens: &[Token]) {
    for token in tokens {
        println!("{}", token);
    }
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

pub fn is_equal(token: &Token) -> bool {
    token.word == "="
}

pub fn is_plus(token: &Token) -> bool {
    token.word == "+"
}

pub fn is_minus(token: &Token) -> bool {
    token.word == "-"
}
