use std::env;

mod error;
mod token;
mod lexical_analize;
mod syntax_analize;
mod equation;
mod maths;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(error::commandline_arg().into())
    }
    let tokens = match lexical_analize::tokenize(&args[1]) {
        Ok(t) => t,
        Err(e) => return Err(e.into()),
    };
    if let Err(e) = syntax_analize::check_syntax(&tokens) {
        return Err(e.into())
    };
    let eq_components = match equation::parsing::decompose(&tokens) {
        Err(e) => return Err(e.into()),
        Ok(c) => c
    };
    equation::display_reduced_eq(&eq_components);
    if let Err(e) = equation::solver::display_rslt(&eq_components) {
        return Err(e.into());
    }
    Ok(())
}
