#[cfg(test)]
mod test {
    use crate::syntax_analize;
    use crate::lexical_analize;

    fn run_one_ok_test(s: &str) {
        let token_vec = match lexical_analize::tokenize(s.to_string()) {
            Ok(t) => t,
            Err(_) => return,
        };
       assert!(syntax_analize::check_syntax(&token_vec).is_ok());
    }

    fn run_one_failed_test(s: &str) {
        let token_vec = match lexical_analize::tokenize(s.to_string()) {
            Ok(t) => t,
            Err(_) => return,
        };
       assert!(syntax_analize::check_syntax(&token_vec).is_err());
    }

    #[test]
    fn test_check_syntax() {
        run_one_ok_test("X = 0");
        run_one_ok_test("12 = 1");
        run_one_ok_test("-12 = 0");
        run_one_ok_test("X^1 = 0");
        run_one_ok_test("X + 2 = 0");
        run_one_ok_test("-123 * X^5 = 0");
        run_one_ok_test("3 = 2 * X^1 - +4 * X + -2 * X^2");
        run_one_ok_test("X * 4 = 4 * X + 5 + 6 * X + X + 3 * X");
        run_one_ok_test("-1 * X - 12 * X^2 + X^1244 + -12 * X = 0");
        run_one_ok_test("3 + 2 = 0");

        run_one_failed_test("* = 0");
        run_one_failed_test("+ = 0");
        run_one_failed_test("X * X = 0");
        run_one_failed_test("X^3 * 3 * X = 0");
        run_one_failed_test("1 = X^2 *");
        run_one_failed_test("1 = 4 *");
        run_one_failed_test("0 = * 4");
        run_one_failed_test("0 = 4 +");
        run_one_failed_test("+ 4 = 0");
        run_one_failed_test("X * X^1 = 0");
        run_one_failed_test("X * X + 4 * X = 0");
        run_one_failed_test("3 + + 3 = 0");
        run_one_failed_test("- 4 = 0");
        run_one_failed_test("-1 * X - 12 * X^2 + X^1244 = -12 * X * X");
        run_one_failed_test("= X + 0");
        run_one_failed_test("1 = X + 0 = 12");
        run_one_failed_test("X + 0");
        run_one_failed_test("X + 0 = - 1");
        run_one_failed_test("X + 0 = * 1");
        run_one_failed_test("X + 0 =");
        run_one_failed_test("=");
        run_one_failed_test("1 * = 0 ");
        run_one_failed_test("1 - = 0 ");
        run_one_failed_test("1 + = 0 ");
        run_one_failed_test("0 = 1 = 0");
    }
}
