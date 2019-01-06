#[cfg(test)]
mod test {
    use crate::lexical_analize;
    use crate::token;

    #[test]
    fn test_get_token_role() {
        assert_eq!(lexical_analize::get_token_role("+".to_string()),                     token::Type::SeparationOp);
        assert_eq!(lexical_analize::get_token_role("++".to_string()),                    token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("+-".to_string()),                    token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("+-".to_string()),                    token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("*".to_string()),                     token::Type::FactorOp);
        assert_eq!(lexical_analize::get_token_role("**".to_string()),                    token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("3".to_string()),                     token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("-3".to_string()),                    token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("+3".to_string()),                    token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("3u".to_string()),                    token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("3.".to_string()),                    token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("13.2".to_string()),                  token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("-1344.2444484948484".to_string()),   token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("44.24dd4444".to_string()),           token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role(".254".to_string()),                  token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("-.254".to_string()),                 token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("+.254+".to_string()),                token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("X".to_string()),                     token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^1".to_string()),                   token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^2".to_string()),                   token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^-1".to_string()),                  token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^a".to_string()),                   token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^a1".to_string()),                  token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("XX^3".to_string()),                  token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("-X^3".to_string()),                  token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("/".to_string()),                     token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("%".to_string()),                     token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("".to_string()),                      token::Type::Unknown);
    }

    #[test]
    fn test_check_exponent() {
        assert_eq!(lexical_analize::check_exponent(&"X".to_string()), Ok(1));
        assert_eq!(lexical_analize::check_exponent(&"X^1".to_string()), Ok(1));
        assert_eq!(lexical_analize::check_exponent(&"X^0".to_string()), Ok(0));
        assert_eq!(lexical_analize::check_exponent(&"X^+1".to_string()), Ok(1));
        assert_eq!(lexical_analize::check_exponent(&"X^13".to_string()), Ok(13));
        assert_eq!(lexical_analize::check_exponent(&"X^1352882".to_string()), Ok(1352882));

        assert!(lexical_analize::check_exponent(&"".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^^".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^X".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^-1".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^++1".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^a1".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^2.5".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^-2.5".to_string()).is_err());
        assert!(lexical_analize::check_exponent(&"X^1/5".to_string()).is_err());
    }

    #[test]
    fn test_tokenize() {
        assert!(lexical_analize::tokenize("X".to_string()).is_ok());
        assert!(lexical_analize::tokenize("X + 1".to_string()).is_ok());
        assert!(lexical_analize::tokenize("X^1 + 1".to_string()).is_ok());
        assert!(lexical_analize::tokenize("+ 1".to_string()).is_ok());
        assert!(lexical_analize::tokenize("- 1".to_string()).is_ok());
        assert!(lexical_analize::tokenize("-1 - X^2 * * -1".to_string()).is_ok());
        assert!(lexical_analize::tokenize("+3 - X^2 * - 1".to_string()).is_ok());
        assert!(lexical_analize::tokenize("123 + X^1 + - 1".to_string()).is_ok());
        assert!(lexical_analize::tokenize("3 + 2 * X^1 - +4 * X^8 + -2 * X^2".to_string()).is_ok());

        assert!(lexical_analize::tokenize("".to_string()).is_err());
        assert!(lexical_analize::tokenize("X^-1 + 1".to_string()).is_err());
        assert!(lexical_analize::tokenize("X^2 ** -1".to_string()).is_err());
        assert!(lexical_analize::tokenize("X^2a * * -1".to_string()).is_err());
        assert!(lexical_analize::tokenize("X^2 * 21a * -1".to_string()).is_err());
        assert!(lexical_analize::tokenize("2 + ++3".to_string()).is_err());
        assert!(lexical_analize::tokenize("X ^ 2 + 3".to_string()).is_err());
        assert!(lexical_analize::tokenize("X ^ 2 + 3".to_string()).is_err());
        assert!(lexical_analize::tokenize("3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2".to_string()).is_err());
        assert!(lexical_analize::tokenize("X ^ 2 + 3".to_string()).is_err());
    }
}
