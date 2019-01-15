#[cfg(test)]
mod test {
    use crate::lexical_analize;
    use crate::token;

    #[test]
    fn test_get_token_role() {
        assert_eq!(lexical_analize::get_token_role("+"),                     token::Type::SeparationOp);
        assert_eq!(lexical_analize::get_token_role("++"),                    token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("+-"),                    token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("+-"),                    token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("*"),                     token::Type::FactorOp);
        assert_eq!(lexical_analize::get_token_role("**"),                    token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("3"),                     token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("-3"),                    token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("+3"),                    token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("3u"),                    token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("3."),                    token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("13.2"),                  token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("-1344.2444484948484"),   token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("44.24dd4444"),           token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role(".254"),                  token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("-.254"),                 token::Type::Coefficient);
        assert_eq!(lexical_analize::get_token_role("+.254+"),                token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("X"),                     token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^1"),                   token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^2"),                   token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^-1"),                  token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^a"),                   token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("X^a1"),                  token::Type::Indeterminate);
        assert_eq!(lexical_analize::get_token_role("XX^3"),                  token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("-X^3"),                  token::Type::Unknown);

        assert_eq!(lexical_analize::get_token_role("/"),                     token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role("%"),                     token::Type::Unknown);
        assert_eq!(lexical_analize::get_token_role(""),                      token::Type::Unknown);
    }

    fn run_check_expo_test_ok(s: &str, rslt: u32) {
        let t = token::Token { word: s.to_string(), role: token::Type::Unknown, exponent: 1 };
        assert_eq!(lexical_analize::check_exponent(&t), Ok(rslt))
    }

    fn run_check_expo_test_failure(s: &str) {
        let t = token::Token { word: s.to_string(), role: token::Type::Unknown, exponent: 1 };
        assert!(lexical_analize::check_exponent(&t).is_err())
    }

    #[test]
    fn test_check_exponent() {
        run_check_expo_test_ok("X", 1);
        run_check_expo_test_ok("X^1", 1);
        run_check_expo_test_ok("X^0", 0);
        run_check_expo_test_ok("X^+1", 1);
        run_check_expo_test_ok("X^13", 13);
        run_check_expo_test_ok("X^1352882", 1352882);

        run_check_expo_test_failure("");
        run_check_expo_test_failure("X^");
        run_check_expo_test_failure("X^^");
        run_check_expo_test_failure("X^X");
        run_check_expo_test_failure("X^-1");
        run_check_expo_test_failure("X^++1");
        run_check_expo_test_failure("X^a1");
        run_check_expo_test_failure("X^2.5");
        run_check_expo_test_failure("X^-2.5");
        run_check_expo_test_failure("X^1/5");
    }

    #[test]
    fn test_tokenize() {
        assert!(lexical_analize::tokenize("X = 0").is_ok());
        assert!(lexical_analize::tokenize("X = 1").is_ok());
        assert!(lexical_analize::tokenize("X^1 + 1 = 0").is_ok());
        assert!(lexical_analize::tokenize("+ 1").is_ok());
        assert!(lexical_analize::tokenize("- 1").is_ok());
        assert!(lexical_analize::tokenize("-1 - X^2 * * -1 = 0").is_ok());
        assert!(lexical_analize::tokenize("+3 = X^2 * - 1").is_ok());
        assert!(lexical_analize::tokenize("123 + X^1 = -1").is_ok());
        assert!(lexical_analize::tokenize("3  2 * X^1 - +4 * X^8 + -2 * X^2 = 0").is_ok());
        assert!(lexical_analize::tokenize("= 123 + X^1 = -1").is_ok());
        assert!(lexical_analize::tokenize("=").is_ok());

        assert!(lexical_analize::tokenize("").is_err());
        assert!(lexical_analize::tokenize("X^-1 + 1 = 0").is_err());
        assert!(lexical_analize::tokenize("X^2 = ** -1").is_err());
        assert!(lexical_analize::tokenize("X^2a * * -1").is_err());
        assert!(lexical_analize::tokenize("X^2 * 21a * -1 = 0").is_err());
        assert!(lexical_analize::tokenize("2 + ++3 = 0").is_err());
        assert!(lexical_analize::tokenize("X ^ 2 + 3").is_err());
        assert!(lexical_analize::tokenize("X ^ 2 + 3 = 0").is_err());
        assert!(lexical_analize::tokenize("3.5678 ++ 2a * X^1 - +4 * X + -02 * X^2").is_err());
        assert!(lexical_analize::tokenize("X ^ 2 + 3 = 0").is_err());
        assert!(lexical_analize::tokenize("X ^ 2 + 3 == 0").is_err());
    }
}
