use {
    crate::{error::SourceRange, identifier::Identifier},
    std::{
        fmt::{Display, Formatter, Result},
        path::PathBuf,
    },
};

// Keywords
pub const AS_KEYWORD: &str = "as";
pub const BOOL_KEYWORD: &str = "bool";
pub const BYTES_KEYWORD: &str = "bytes";
pub const CHOICE_KEYWORD: &str = "choice";
pub const F64_KEYWORD: &str = "f64";
pub const IMPORT_KEYWORD: &str = "import";
pub const OPTIONAL_KEYWORD: &str = "optional";
pub const S64_KEYWORD: &str = "s64";
pub const STRING_KEYWORD: &str = "string";
pub const STRUCT_KEYWORD: &str = "struct";
pub const U64_KEYWORD: &str = "u64";
pub const UNIT_KEYWORD: &str = "unit";
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
    Bool,
    Bytes,
    Choice,
    Colon,
    Dot,
    Equals,
    F64,
    Identifier(Identifier),
    Import,
    Integer(usize),
    LeftCurly,
    LeftSquare,
    Optional,
    Path(PathBuf),
    RightCurly,
    RightSquare,
    S64,
    String,
    Struct,
    U64,
    Unit,
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
            Self::Bool => write!(f, "{}", BOOL_KEYWORD),
            Self::Bytes => write!(f, "{}", BYTES_KEYWORD),
            Self::Choice => write!(f, "{}", CHOICE_KEYWORD),
            Self::Colon => write!(f, ":"),
            Self::Dot => write!(f, "."),
            Self::Equals => write!(f, "="),
            Self::F64 => write!(f, "{}", F64_KEYWORD),
            Self::Identifier(name) => write!(f, "{}", name.original()),
            Self::Import => write!(f, "{}", IMPORT_KEYWORD),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::LeftCurly => write!(f, "{{"),
            Self::LeftSquare => write!(f, "["),
            Self::Optional => write!(f, "{}", OPTIONAL_KEYWORD),
            Self::Path(path) => write!(f, "'{}'", path.display()),
            Self::RightCurly => write!(f, "}}"),
            Self::RightSquare => write!(f, "]"),
            Self::S64 => write!(f, "{}", S64_KEYWORD),
            Self::String => write!(f, "{}", STRING_KEYWORD),
            Self::Struct => write!(f, "{}", STRUCT_KEYWORD),
            Self::U64 => write!(f, "{}", U64_KEYWORD),
            Self::Unit => write!(f, "{}", UNIT_KEYWORD),
            Self::Unstable => write!(f, "{}", UNSTABLE_KEYWORD),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::SourceRange,
        token::{
            Token, Variant, AS_KEYWORD, BOOL_KEYWORD, BYTES_KEYWORD, CHOICE_KEYWORD, F64_KEYWORD,
            IMPORT_KEYWORD, OPTIONAL_KEYWORD, S64_KEYWORD, STRING_KEYWORD, STRUCT_KEYWORD,
            U64_KEYWORD, UNIT_KEYWORD, UNSTABLE_KEYWORD,
        },
    };
    use std::path::Path;

    #[test]
    fn token_display() {
        assert_eq!(
            format!(
                "{}",
                Token {
                    source_range: SourceRange { start: 0, end: 2 },
                    variant: Variant::As,
                },
            ),
            AS_KEYWORD,
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
    fn variant_bytes_display() {
        assert_eq!(format!("{}", Variant::Bytes), BYTES_KEYWORD);
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
    fn variant_f64_display() {
        assert_eq!(format!("{}", Variant::F64), F64_KEYWORD);
    }

    #[test]
    fn variant_identifier_display() {
        assert_eq!(format!("{}", Variant::Identifier("foo".into())), "foo");
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
    fn variant_left_square_display() {
        assert_eq!(format!("{}", Variant::LeftSquare), "[");
    }

    #[test]
    fn variant_optional_display() {
        assert_eq!(format!("{}", Variant::Optional), OPTIONAL_KEYWORD);
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
    fn variant_right_square_display() {
        assert_eq!(format!("{}", Variant::RightSquare), "]");
    }

    #[test]
    fn variant_s64_display() {
        assert_eq!(format!("{}", Variant::S64), S64_KEYWORD);
    }

    #[test]
    fn variant_string_display() {
        assert_eq!(format!("{}", Variant::String), STRING_KEYWORD);
    }

    #[test]
    fn variant_struct_display() {
        assert_eq!(format!("{}", Variant::Struct), STRUCT_KEYWORD);
    }

    #[test]
    fn variant_u64_display() {
        assert_eq!(format!("{}", Variant::U64), U64_KEYWORD);
    }

    #[test]
    fn variant_unit_display() {
        assert_eq!(format!("{}", Variant::Unit), UNIT_KEYWORD);
    }

    #[test]
    fn variant_unstable_display() {
        assert_eq!(format!("{}", Variant::Unstable), UNSTABLE_KEYWORD);
    }
}
