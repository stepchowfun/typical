use colored::{ColoredString, Colorize, control::SHOULD_COLORIZE};
use std::path::Path;

// This trait formats a filesystem path for human-facing diagnostic output.
pub trait CodePath {
    fn code_path(&self) -> ColoredString;
}

impl CodePath for Path {
    fn code_path(&self) -> ColoredString {
        self.to_string_lossy().code_str()
    }
}

// This trait has a function for formatting "code-like" text, such as a file path. The reason it's
// implemented as a trait and not just a function is so we can use it with method syntax, as in
// `x.code_str()`. Rust does not allow us to implement methods on primitive types such as `str`.
pub trait CodeStr {
    fn code_str(&self) -> ColoredString;
}

impl CodeStr for str {
    fn code_str(&self) -> ColoredString {
        // If colored output is enabled, format nonempty text in magenta. Otherwise, surround it in
        // backticks so even an empty string remains visible.
        if !self.is_empty() && SHOULD_COLORIZE.should_colorize() {
            self.magenta()
        } else {
            ColoredString::from(&format!("`{self}`") as &Self)
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
