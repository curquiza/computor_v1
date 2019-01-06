mod token;
mod lexical_analize;
mod syntax_analize;
mod equation;

fn exit_with_error(error: String) -> Result<(), Box<std::error::Error>> {
    eprintln!("{}", error);
    Err(error.into())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let s = "3 + 2 * X^1 - +4 * X^8 + -2 * X^2".to_string();
    // let s = "   ".to_string();
    // let s = "3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2".to_string();

    println!("{}", s);
    let tokens = match lexical_analize::tokenize(s) {
        Ok(t) => t,
        Err(e) => return exit_with_error(e),
    };
    if let Err(e) = syntax_analize::check_syntax(&tokens) {
        return exit_with_error(e)
    };
    if let Err(e) = equation::get_degree(&tokens) {
        return exit_with_error(e)
    };
    Ok(())

    // TODO: parsing
    // TODO: resoudre
    // TODO: implenter display pour token
    // TODO: retoun erreur du main chelou avec .into() -> trouver good way pour retourner des err
}
