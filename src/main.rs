use std::env;

mod error;
mod token;
mod lexical_analize;
mod syntax_analize;
mod equation;

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
    // if let Err(e) = equation::get_degree(&tokens) {
    //     return exit_with_error(e)
    // };
    Ok(())

    // TODO: parsing + afficher au format reduit
    // TODO: resoudre
    // TODO: implenter display pour token
    // TODO: retoun erreur du main chelou avec .into() -> trouver good way pour retourner des err
}
