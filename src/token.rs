use crate::{error::SourceRange, identifier::Identifier};
use std::{
    fmt::{Display, Formatter, Result},
    path::PathBuf,
};

// Keywords
pub const AS_KEYWORD: &str = "as";
pub const BOOL_KEYWORD: &str = "Boolean";
pub const CHOICE_KEYWORD: &str = "choice";
pub const IMPORT_KEYWORD: &str = "import";
pub const STRUCT_KEYWORD: &str = "struct";
pub const UNSTABLE_KEYWORD: &str = "unstable";

// The first step of compilation is to split the source into a stream of tokens. This struct
// represents a single token.
#[derive(Clone, Debug)]
pub struct Token {
    pub source_range: SourceRange,
    pub variant: Variant,
}

// We assign each token a "variant" describing what kind of token it is.
#[derive(Clone, Debug)]
pub enum Variant {
    As,
    Boolean,
    Choice,
    Colon,
    Dot,
    Equals,
    Identifier(Identifier),
    Import,
    Integer(usize),
    LeftCurly,
    Path(PathBuf),
    RightCurly,
    Struct,
    Unstable,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.variant)
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::As => write!(f, "{}", AS_KEYWORD),
            Self::Boolean => write!(f, "{}", BOOL_KEYWORD),
            Self::Choice => write!(f, "{}", CHOICE_KEYWORD),
            Self::Colon => write!(f, ":"),
            Self::Dot => write!(f, "."),
            Self::Equals => write!(f, "="),
            Self::Identifier(name) => write!(f, "{}", name.original()),
            Self::Import => write!(f, "{}", IMPORT_KEYWORD),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::LeftCurly => write!(f, "{{"),
            Self::Path(path) => write!(f, "'{}'", path.display()),
            Self::RightCurly => write!(f, "}}"),
            Self::Struct => write!(f, "{}", STRUCT_KEYWORD),
            Self::Unstable => write!(f, "{}", UNSTABLE_KEYWORD),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::SourceRange,
        token::{
            Token, Variant, AS_KEYWORD, BOOL_KEYWORD, CHOICE_KEYWORD, IMPORT_KEYWORD,
            STRUCT_KEYWORD, UNSTABLE_KEYWORD,
        },
    };
    use std::path::Path;

    #[test]
    fn token_display() {
        assert_eq!(
            format!(
                "{}",
                Token {
                    source_range: SourceRange { start: 0, end: 0 },
                    variant: Variant::Choice,
                },
            ),
            CHOICE_KEYWORD,
        );
    }

    #[test]
    fn variant_as_display() {
        assert_eq!(format!("{}", Variant::As), AS_KEYWORD);
    }

    #[test]
    fn variant_bool_display() {
        assert_eq!(format!("{}", Variant::Boolean), BOOL_KEYWORD);
    }

    #[test]
    fn variant_choice_display() {
        assert_eq!(format!("{}", Variant::Choice), CHOICE_KEYWORD);
    }

    #[test]
    fn variant_colon_display() {
        assert_eq!(format!("{}", Variant::Colon), ":");
    }

    #[test]
    fn variant_dot_display() {
        assert_eq!(format!("{}", Variant::Dot), ".");
    }

    #[test]
    fn variant_equals_display() {
        assert_eq!(format!("{}", Variant::Equals), "=");
    }

    #[test]
    fn variant_identifier_display() {
        assert_eq!(format!("{}", Variant::Identifier("Foo".into())), "Foo");
    }

    #[test]
    fn variant_import_display() {
        assert_eq!(format!("{}", Variant::Import), IMPORT_KEYWORD);
    }

    #[test]
    fn variant_integer_literal_display() {
        assert_eq!(format!("{}", Variant::Integer(42)), "42");
    }

    #[test]
    fn variant_left_curly_display() {
        assert_eq!(format!("{}", Variant::LeftCurly), "{");
    }

    #[test]
    fn variant_path_display() {
        assert_eq!(
            format!("{}", Variant::Path(Path::new("foo").to_owned())),
            "'foo'",
        );
    }

    #[test]
    fn variant_right_curly_display() {
        assert_eq!(format!("{}", Variant::RightCurly), "}");
    }

    #[test]
    fn variant_struct_display() {
        assert_eq!(format!("{}", Variant::Struct), STRUCT_KEYWORD);
    }

    #[test]
    fn variant_unstable_display() {
        assert_eq!(format!("{}", Variant::Unstable), UNSTABLE_KEYWORD);
    }
}
