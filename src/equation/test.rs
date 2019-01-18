#[cfg(test)]
mod test {
    use crate::lexical_analize;
    use crate::equation;

    // fn run_one_ok_test(s: &str, rslt: u32) {
    //     let token_vec = match lexical_analize::tokenize(s.to_string()) {
    //         Ok(t) => t,
    //         Err(_) => return,
    //     };
    //    assert_eq!(equation::get_degree(&token_vec), Ok(rslt));
    // }
    //
    // fn run_one_failed_test(s: &str) {
    //     let token_vec = match lexical_analize::tokenize(s.to_string()) {
    //         Ok(t) => t,
    //         Err(_) => return,
    //     };
    //    assert!(equation::get_degree(&token_vec).is_err());
    // }

    // #[test]
    // fn test_get_degree() {
    //     run_one_ok_test("X = 0", 1);
    //     run_one_ok_test("X^2 = 0", 2);
    //     run_one_ok_test("3 - X^0 = 0", 0);
    //     run_one_ok_test("3 + X^2 + X = 0", 2);
    //     run_one_ok_test("3 = 0", 0);
    //     run_one_ok_test("1 = X + 3", 1);
    //     run_one_ok_test("1 * X^1 - X = 3", 1);
    //     run_one_ok_test("1 * X + 3 - 8 = -9 * X^2", 2);
    //
    //     run_one_failed_test("");
    //     run_one_failed_test("X^3 = 0");
    //     run_one_failed_test("3 + X^8 + X = 0");
    //     run_one_failed_test("1 * X^4 - X + 3 = 0");
    // }

    fn run_one_display_test(s: &str, rslt: &str) {
        let token_vec = match lexical_analize::tokenize(s) {
            Ok(t) => t,
            Err(_) => return,
        };
        let components = equation::parse(&token_vec);
        assert_eq!(equation::get_reduced_form(&components), rslt);
    }

    #[test]
    fn test_get_reduced_form() {
        run_one_display_test("0 = 0", "0");
        run_one_display_test("X^0 = 0", "1");
        run_one_display_test("X = 0", "X");
        run_one_display_test("1 * X = 0", "X");
        run_one_display_test("1 * X^1 = 0", "X");
        run_one_display_test("X^1 = 0", "X");
        run_one_display_test("X^0 * 12.5 = 0", "12.5");
        run_one_display_test("X^8 * -0.0 = 0", "0");
        run_one_display_test("X^8 * 0.0 = 0", "0");
        run_one_display_test("X^0 = 1", "0");
        run_one_display_test("12 = 12", "0");
        run_one_display_test("X^0 = 2", "-1");
        run_one_display_test("0 * X^0 = 2", "-2");
        run_one_display_test("2 * X^8 + -5 * X^0 - -4 * X^17 + 12.5 = 2.5 * X^8", "7.5 + -0.5 * X^8 + 4 * X^17");
        run_one_display_test("2 * X^1 - X^0 * 13 = 1", "-14 + 2 * X");
        run_one_display_test("-4 * X^3 - X^1 * 2 + 4 = 0", "4 + -2 * X + -4 * X^3");
        run_one_display_test("-1 * X = X", "-2 * X");
        run_one_display_test("X^0 - -1 * X = X", "1");
        run_one_display_test("-1 * X^0 - X * -1 = X", "-1");
    }
}
