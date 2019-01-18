#[cfg(test)]
mod test {
    use crate::lexical_analize;
    use crate::equation;

    fn run_reduced_form_test(s: &str, rslt: &str) {
        let token_vec = match lexical_analize::tokenize(s) {
            Ok(t) => t,
            Err(_) => return,
        };
        let components = equation::parse(&token_vec);
        assert_eq!(equation::get_reduced_form(&components), rslt);
    }

    #[test]
    fn test_get_reduced_form() {
        run_reduced_form_test("0 = 0", "0");
        run_reduced_form_test("X^0 = 0", "1");
        run_reduced_form_test("X = 0", "X");
        run_reduced_form_test("1 * X = 0", "X");
        run_reduced_form_test("1 * X^1 = 0", "X");
        run_reduced_form_test("X^1 = 0", "X");
        run_reduced_form_test("X^0 * 12.5 = 0", "12.5");
        run_reduced_form_test("X^8 * -0.0 = 0", "0");
        run_reduced_form_test("X^8 * 0.0 = 0", "0");
        run_reduced_form_test("X^0 = 1", "0");
        run_reduced_form_test("12 = 12", "0");
        run_reduced_form_test("X^0 = 2", "-1");
        run_reduced_form_test("0 * X^0 = 2", "-2");
        run_reduced_form_test("2 * X^8 + -5 * X^0 - -4 * X^17 + 12.5 = 2.5 * X^8", "7.5 + -0.5 * X^8 + 4 * X^17");
        run_reduced_form_test("2 * X^1 - X^0 * 13 = 1", "-14 + 2 * X");
        run_reduced_form_test("-4 * X^3 - X^1 * 2 + 4 = 0", "4 + -2 * X + -4 * X^3");
        run_reduced_form_test("-1 * X = X", "-2 * X");
        run_reduced_form_test("X^0 - -1 * X = X", "1");
        run_reduced_form_test("-1 * X^0 - X * -1 = X", "-1");
        run_reduced_form_test("X * -1 - 0 = X", "-2 * X");
    }

    fn run_get_polynomial_degree_test(s: &str, rslt: u32) {
        let token_vec = match lexical_analize::tokenize(s) {
            Ok(t) => t,
            Err(_) => return,
        };
        let components = equation::parse(&token_vec);
        assert_eq!(equation::get_polynomial_degree(&components), rslt);
    }

    #[test]
    fn test_get_polynomial_degree() {
        run_get_polynomial_degree_test("1 = 0", 0);
        run_get_polynomial_degree_test("0 = 0", 0);
        run_get_polynomial_degree_test("X = 0", 1);
        run_get_polynomial_degree_test("X + 1 = 0", 1);
        run_get_polynomial_degree_test("X^345 * 0 + X^1 + 1 = 0", 1);
        run_get_polynomial_degree_test("X^345 * 0 = 1", 0);
        run_get_polynomial_degree_test("X^8 * 1 = 1", 8);
        run_get_polynomial_degree_test("X^8 * 0 = X^2", 2);
        run_get_polynomial_degree_test("3 * X^1 + X - 0 = 1 + X^2", 2);
    }
}
