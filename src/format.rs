use colored::{control::SHOULD_COLORIZE, ColoredString, Colorize};

// This trait has a function for formatting "code-like" text, such as a file path. The reason it's
// implemented as a trait and not just a function is so we can use it with method syntax, as in
// `x.code_str()`. Rust does not allow us to implement methods on primitive types such as `str`.
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
    use crate::format::CodeStr;

    #[test]
    fn code_str_display() {
        // This test, like many others, depends on colors being disabled [ref:colorless_tests].
        assert_eq!(format!("{}", "foo".code_str()), "`foo`");
    }
}
