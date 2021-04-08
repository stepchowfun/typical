// This macro is useful for writing tests that deal with errors. It takes an expression of type
// `Result<_, Vec<Error>>` and a search string and asserts that the expressions evaluates to an
// `Err(_)` and that the string representation of at least one of the errors contains the given
// search string.
#[macro_export]
macro_rules! assert_fails {
    ($expr:expr, $search_str:expr $(,)?) => {{
        // Macros are call-by-name, but we want call-by-value (or at least call-by-need) to avoid
        // accidentally evaluating arguments multiple times. Here we force eager evaluation.
        let expr = $expr;
        let search_str = $search_str;

        // Check that `$expr` fails and that the failure contains `$search_str`.
        if let Err(errors) = expr {
            let mut found_error = false;
            let mut all_errors_string = "".to_owned();

            for error in errors {
                let error_string = error.to_string();
                all_errors_string.push_str(&format!("{}\n", error_string));

                if error_string.contains(search_str) {
                    found_error = true;
                }
            }

            assert!(
                found_error,
                "The expression failed as expected, but not with the expected error.",
            );
        } else {
            panic!("The expression was supposed to fail, but it succeeded.");
        }
    }};
}

// This macro is useful for writing equality tests for types that implement `Debug` but not `Eq`.
// It asserts that the debug representations of the two given expressions match.
#[macro_export]
macro_rules! assert_same {
    ($expr1:expr, $expr2:expr $(,)?) => {{
        // Macros are call-by-name, but we want call-by-value (or at least call-by-need) to avoid
        // accidentally evaluating arguments multiple times. Here we force eager evaluation.
        let expr1 = $expr1;
        let expr2 = $expr2;

        // To aid in type inference, the following tells the compiler that the two expressions have
        // the same type.
        let mut _dummy = &expr1;
        _dummy = &expr2;

        // Assert that the expressions have the same debug representation.
        assert_eq!(format!("{:?}", expr1), format!("{:?}", expr2));
    }};
}

#[cfg(test)]
mod tests {
    use crate::{assert_fails, assert_same, error::Error};

    #[test]
    #[should_panic(expected = "The expression was supposed to fail, but it succeeded.")]
    fn assert_fails_empty() {
        let success: Result<usize, Vec<Error>> = Ok(42);

        assert_fails!(success, "search string");
    }

    #[test]
    fn assert_fails_match() {
        let success: Result<usize, Vec<Error>> = Err(vec![
            Error {
                message: "foo bar".to_owned(),
                reason: None,
            },
            Error {
                message: "foo search string bar".to_owned(),
                reason: None,
            },
            Error {
                message: "foo bar".to_owned(),
                reason: None,
            },
        ]);

        assert_fails!(success, "search string");
    }

    #[test]
    #[should_panic(
        // This comma on the comment at the end of the line below is needed to satisfy the trailing
        // commas check.
        expected = "The expression failed as expected, but not with the expected error." // ,
    )]
    fn assert_fails_mismatch() {
        let success: Result<usize, Vec<Error>> = Err(vec![
            Error {
                message: "foo bar".to_owned(),
                reason: None,
            },
            Error {
                message: "foo bar".to_owned(),
                reason: None,
            },
            Error {
                message: "foo bar".to_owned(),
                reason: None,
            },
        ]);

        assert_fails!(success, "search string");
    }

    #[test]
    fn assert_same_match() {
        assert_same!(42, 42);
    }

    #[test]
    #[should_panic(expected = "42")]
    fn assert_same_mismatch() {
        assert_same!(42, 43);
    }
}
