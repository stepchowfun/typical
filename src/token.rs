use std::fmt::{Display, Formatter, Result};

// Keywords
pub const ENUM_KEYWORD: &str = "enum";
pub const MIGRATING_KEYWORD: &str = "migrating";
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
    Colon,
    Enum,
    Equals,
    Identifier(&'a str),
    IntegerLiteral(u64),
    LeftCurly,
    Migrating,
    RightCurly,
    Separator,
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
            Self::Colon => write!(f, ":"),
            Self::Enum => write!(f, "{}", ENUM_KEYWORD),
            Self::Equals => write!(f, "="),
            Self::Identifier(name) => write!(f, "{}", name),
            Self::IntegerLiteral(integer) => write!(f, "{}", integer),
            Self::LeftCurly => write!(f, "{{"),
            Self::Migrating => write!(f, "{}", MIGRATING_KEYWORD),
            Self::RightCurly => write!(f, "}}"),
            Self::Separator => write!(f, "\\n"),
            Self::Struct => write!(f, "{}", STRUCT_KEYWORD),
        }
    }
}
