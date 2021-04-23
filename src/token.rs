use crate::{error::SourceRange, naming_conventions::snake_case};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
    path::PathBuf,
};

// Keywords
pub const AS_KEYWORD: &str = "as";
pub const BOOL_KEYWORD: &str = "Bool";
pub const CHOICE_KEYWORD: &str = "choice";
pub const IMPORT_KEYWORD: &str = "import";
pub const TRANSITIONAL_KEYWORD: &str = "transitional";
pub const STRUCT_KEYWORD: &str = "struct";

// The first step of compilation is to split the source into a stream of tokens. This struct
// represents a single token.
#[derive(Clone, Debug)]
pub struct Token {
    pub source_range: SourceRange,
    pub variant: Variant,
}

// This is a user-provided identifier. To make case-insensitive equality, hashing, etc.
// performance, we pre-compute a case-folded version of the identifier. The main purpose of this
// struct (rather than just using `String`) is to prevent accidental inclusion of identifiers in
// generated code without case conversion.
#[derive(Clone, Debug)]
pub struct Identifier {
    original: String,
    case_folded: String,
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.case_folded == other.case_folded
    }
}

impl Eq for Identifier {}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.case_folded.partial_cmp(&other.case_folded)
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.case_folded.cmp(&other.case_folded)
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.case_folded.hash(state);
    }
}

impl From<&str> for Identifier {
    fn from(string: &str) -> Self {
        Identifier {
            original: string.to_owned(),
            case_folded: snake_case(string),
        }
    }
}

impl Identifier {
    pub fn original(&self) -> &str {
        &self.original
    }
}

// We assign each token a "variant" describing what kind of token it is.
#[derive(Clone, Debug)]
pub enum Variant {
    As,
    Bool,
    Choice,
    Colon,
    Dot,
    Equals,
    Identifier(Identifier),
    Import,
    Integer(usize),
    LeftCurly,
    Path(PathBuf),
    Transitional,
    RightCurly,
    Struct,
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
            Self::Bool => write!(f, "{}", BOOL_KEYWORD),
            Self::Choice => write!(f, "{}", CHOICE_KEYWORD),
            Self::Colon => write!(f, ":"),
            Self::Dot => write!(f, "."),
            Self::Equals => write!(f, "="),
            Self::Identifier(name) => write!(f, "{}", name.original),
            Self::Import => write!(f, "{}", IMPORT_KEYWORD),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::LeftCurly => write!(f, "{{"),
            Self::Path(path) => write!(f, "'{}'", path.display()),
            Self::Transitional => write!(f, "{}", TRANSITIONAL_KEYWORD),
            Self::RightCurly => write!(f, "}}"),
            Self::Struct => write!(f, "{}", STRUCT_KEYWORD),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::SourceRange,
        token::{
            Token, Variant, AS_KEYWORD, BOOL_KEYWORD, CHOICE_KEYWORD, IMPORT_KEYWORD,
            STRUCT_KEYWORD, TRANSITIONAL_KEYWORD,
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
        assert_eq!(format!("{}", Variant::Bool), BOOL_KEYWORD);
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
    fn variant_transitional_display() {
        assert_eq!(format!("{}", Variant::Transitional), TRANSITIONAL_KEYWORD);
    }

    #[test]
    fn variant_right_curly_display() {
        assert_eq!(format!("{}", Variant::RightCurly), "}");
    }

    #[test]
    fn variant_struct_display() {
        assert_eq!(format!("{}", Variant::Struct), STRUCT_KEYWORD);
    }
}
