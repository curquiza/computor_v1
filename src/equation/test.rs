#[cfg(test)]
mod test {
    use crate::lexical_analize;
    use crate::equation;

    fn run_one_ok_test(s: &str, rslt: u32) {
        let token_vec = match lexical_analize::tokenize(s.to_string()) {
            Ok(t) => t,
            Err(_) => return,
        };
       assert_eq!(equation::get_degree(&token_vec), Ok(rslt));
    }

    fn run_one_failed_test(s: &str) {
        let token_vec = match lexical_analize::tokenize(s.to_string()) {
            Ok(t) => t,
            Err(_) => return,
        };
       assert!(equation::get_degree(&token_vec).is_err());
    }

    #[test]
    fn test_get_degree() {
        run_one_ok_test("X", 1);
        run_one_ok_test("X^2", 2);
        run_one_ok_test("3 - X^0", 0);
        run_one_ok_test("3 + X^2 + X", 2);
        run_one_ok_test("3", 0);
        run_one_ok_test("1 + X + 3", 1);
        run_one_ok_test("1 * X^1 - X + 3", 1);
        run_one_ok_test("1 * X + 3 - 8 - 9 * X^2", 2);

        run_one_failed_test("");
        run_one_failed_test("X^3");
        run_one_failed_test("3 + X^8 + X");
        run_one_failed_test("1 * X^4 - X + 3");
    }
}
