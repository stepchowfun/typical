use std::{
    fmt::{Display, Formatter, Result},
    path::Path,
};

// Keywords
pub const AS_KEYWORD: &str = "as";
pub const CHOICE_KEYWORD: &str = "choice";
pub const IMPORT_KEYWORD: &str = "import";
pub const RESTRICTED_KEYWORD: &str = "restricted";
pub const STRUCT_KEYWORD: &str = "struct";

// The first step of compilation is to split the source into a stream of tokens. This struct
// represents a single token.
#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub source_range: (usize, usize), // Inclusive on the left and exclusive on the right
    pub variant: Variant<'a>,
}

// We assign each token a "variant" describing what kind of token it is.
#[derive(Clone, Debug)]
pub enum Variant<'a> {
    As,
    Choice,
    Colon,
    Dot,
    Equals,
    Identifier(&'a str),
    Import,
    Integer(usize),
    LeftCurly,
    Path(&'a Path),
    Restricted,
    RightCurly,
    Struct,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.variant)
    }
}

impl<'a> Display for Variant<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::As => write!(f, "{}", AS_KEYWORD),
            Self::Choice => write!(f, "{}", CHOICE_KEYWORD),
            Self::Colon => write!(f, ":"),
            Self::Dot => write!(f, "."),
            Self::Equals => write!(f, "="),
            Self::Identifier(name) => write!(f, "{}", name),
            Self::Import => write!(f, "{}", IMPORT_KEYWORD),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::LeftCurly => write!(f, "{{"),
            Self::Path(path) => write!(f, "'{}'", path.display()),
            Self::Restricted => write!(f, "{}", RESTRICTED_KEYWORD),
            Self::RightCurly => write!(f, "}}"),
            Self::Struct => write!(f, "{}", STRUCT_KEYWORD),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_equal,
        token::{
            Token, Variant, AS_KEYWORD, CHOICE_KEYWORD, IMPORT_KEYWORD, RESTRICTED_KEYWORD,
            STRUCT_KEYWORD,
        },
    };
    use std::path::Path;

    #[test]
    fn token_display() {
        assert_equal!(
            format!(
                "{}",
                Token {
                    source_range: (0, 0),
                    variant: Variant::Choice,
                },
            ),
            CHOICE_KEYWORD,
        );
    }

    #[test]
    fn variant_as_display() {
        assert_equal!(format!("{}", Variant::As), AS_KEYWORD);
    }

    #[test]
    fn variant_choice_display() {
        assert_equal!(format!("{}", Variant::Choice), CHOICE_KEYWORD);
    }

    #[test]
    fn variant_colon_display() {
        assert_equal!(format!("{}", Variant::Colon), ":");
    }

    #[test]
    fn variant_dot_display() {
        assert_equal!(format!("{}", Variant::Dot), ".");
    }

    #[test]
    fn variant_equals_display() {
        assert_equal!(format!("{}", Variant::Equals), "=");
    }

    #[test]
    fn variant_identifier_display() {
        assert_equal!(format!("{}", Variant::Identifier("foo")), "foo");
    }

    #[test]
    fn variant_import_display() {
        assert_equal!(format!("{}", Variant::Import), IMPORT_KEYWORD);
    }

    #[test]
    fn variant_integer_literal_display() {
        assert_equal!(format!("{}", Variant::Integer(42)), "42");
    }

    #[test]
    fn variant_left_curly_display() {
        assert_equal!(format!("{}", Variant::LeftCurly), "{");
    }

    #[test]
    fn variant_path_display() {
        assert_equal!(format!("{}", Variant::Path(Path::new("foo"))), "'foo'");
    }

    #[test]
    fn variant_restricted_display() {
        assert_equal!(format!("{}", Variant::Restricted), RESTRICTED_KEYWORD);
    }

    #[test]
    fn variant_right_curly_display() {
        assert_equal!(format!("{}", Variant::RightCurly), "}");
    }

    #[test]
    fn variant_struct_display() {
        assert_equal!(format!("{}", Variant::Struct), STRUCT_KEYWORD);
    }
}
