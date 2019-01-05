mod token;
mod lexical_analize;
mod syntax_analize;

fn exit_with_error(error: String) -> Result<(), Box<std::error::Error>> {
    eprintln!("{}", error);
    Err(error.into())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let s = "3 + 2 * X^1 - +4 * X + -2 * X^2".to_string();
    // let s = "3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2".to_string();

    println!("{}", s);
    let tokens = match lexical_analize::tokenize(s) {
        Ok(t) => t,
        Err(e) => return exit_with_error(e),
    };
    if let Err(e) = syntax_analize::check_syntax(&tokens) {
        return exit_with_error(e);
    }
    Ok(())

    // Exemple tests check syntax
    // X * 4 + 4 * X + 5 + 6 * X + X + 3 * X
    // X * X + 4 * X
    // 4 * X * 2 + 2
    // 4 *
    // +
    // TODO: check exponent du polynome (<= 2)
    // TODO: parsing
    // TODO: resoudre
    // TODO: implenter display pour token
    // TODO: retoun erreur du main chelou avec .into()
}
