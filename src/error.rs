use std::error::Error;
use std::fmt;
use crate::token;

#[derive(Default, PartialEq)]
pub struct AppError {
    pub kind: &'static str,
    pub token: String,
    pub message: &'static str,
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.kind, &self.token, self.message) {
            ("", t, _) if t.is_empty() => write!(f, "{{ message: {:?} }}", self.message),
            (k, t, m) if t.is_empty() => write!(f, "{{ kind: {:?}, message: {:?} }}", k, m),
            ("", t, m) => write!(f, "{{ token: {:?}, message: {:?} }}", t, m),
            (k, t, m) => write!(f, "{{ kind: {:?}, token: {:?}, message: {:?} }}", k, t, m)
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}: {}", self.kind, self.token, self.message)
    }
}

impl Error for AppError {}

/*
** GENERAL ERRORS
*/

pub fn commandline_arg() -> AppError {
    AppError {
        message: "Exactly one command-line argument needed",
        ..Default::default()
    }
}

pub fn empty_arg() -> AppError {
    AppError {
        message: "Empty equation",
        ..Default::default()
    }
}

/*
** LEXICAL ERRORS
*/

pub fn unknown_token(token: &token::Token) -> AppError {
    AppError {
        kind: "Lexical",
        token: token.word.to_owned(),
        message: "Unknown token",
    }
}

pub fn invalid_exponent(token: &token::Token) -> AppError {
    AppError {
        kind: "Lexical",
        token: token.word.to_owned(),
        message: "Invalid exponent",
    }
}

/*
** SYNTAX ERRORS
*/

pub fn equal_count() -> AppError {
    AppError {
        kind: "Syntaxic",
        message: "Equation needs exactly one equal condition",
        ..Default::default()
    }
}

pub fn unexpected_token(token: &token::Token) -> AppError {
    AppError {
        kind: "Syntaxic",
        token: token.word.to_owned(),
        message: "Unexpected token",
    }
}

/*
** SOLVER ERRORS
*/

pub fn too_hight_polynomial_degree() -> AppError {
    AppError {
        kind: "Solver",
        message: "Polynomial degree must be less or equal than 2",
        ..Default::default()
    }
}

pub fn when_solving_degree1_eq() -> AppError {
    AppError {
        kind: "Solver",
        message: "Something went wrong when solving degree 1 equation",
        ..Default::default()
    }
}

pub fn when_solving_degree2_eq() -> AppError {
    AppError {
        kind: "Solver",
        message: "Something went wrong when solving degree 2 equation",
        ..Default::default()
    }
}
