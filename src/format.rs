use colored::{control::SHOULD_COLORIZE, ColoredString, Colorize};

// This trait has a function for formatting "code-like" text, such as a task name or a file path.
// The reason it's implemented as a trait and not just a function is so we can use it with method
// syntax, as in `x.code_str()`. Rust does not allow us to implement methods on primitive types
// such as `str`.
pub trait CodeStr {
    fn code_str(&self) -> ColoredString;
}

impl CodeStr for str {
    fn code_str(&self) -> ColoredString {
        // If colored output is enabled, format the text in magenta. Otherwise, surround it in
        // backticks.
        if SHOULD_COLORIZE.should_colorize() {
            self.magenta()
        } else {
            ColoredString::from(&format!("`{}`", self) as &Self)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_equal, format::CodeStr};

    #[test]
    fn code_str_colorize_off_display() {
        assert_equal!(format!("{}", "foo".code_str()), "`foo`");
    }

    #[test]
    fn code_str_colorize_on_display() {
        // If tests are run in parallel, then this may be overridden by another test (or the other
        // way around). That would cause the test to fail. So we are forced to run tests
        // sequentially. [tag:sequential_tests]
        colored::control::set_override(true);

        assert_eq!(format!("{}", "foo".code_str()), "\u{1b}[35mfoo\u{1b}[0m");
    }
}
