use crate::format::CodeStr;
use colored::ColoredString;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

// This is a case-insensitive identifier. This struct provides two benefits over just using
// `String` directly:
//
//   1. It makes case-insensitive operations like equality testing and comparison cheaper by
//      pre-computing a case-folded version of the identifier.
//   2. It prevents accidental inclusion of non-case-converted identifiers into generated code.
//
// Note that this struct intentionally does not implement `Display` (see (2) above), but it does
// implement `Debug` and `CodeStr`.
#[derive(Clone, Debug)]
pub struct Identifier {
    original: String,
    snake_case: String,
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.snake_case == other.snake_case
    }
}

impl Eq for Identifier {}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.snake_case.partial_cmp(&other.snake_case)
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.snake_case.cmp(&other.snake_case)
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snake_case.hash(state);
    }
}

impl From<&str> for Identifier {
    fn from(string: &str) -> Self {
        Identifier {
            original: string.to_owned(),
            snake_case: split_words(string)
                .iter()
                .map(|word| word.to_lowercase())
                .collect::<Vec<_>>()
                .join("_"),
        }
    }
}

impl CodeStr for Identifier {
    fn code_str(&self) -> ColoredString {
        self.original.code_str()
    }
}

impl Identifier {
    // This function returns the original identifier as provided by the user.
    pub fn original(&self) -> &str {
        &self.original
    }

    // This function returns a `snake_case` version of an identifier.
    pub fn snake_case(&self) -> String {
        self.snake_case.clone()
    }

    // This function returns a `PascalCase` version of an identifier.
    pub fn pascal_case(&self) -> String {
        split_words(&self.original)
            .iter()
            .map(|word| {
                let mut chars = word.chars();

                match chars.next() {
                    None => String::new(),
                    Some(c) => {
                        c.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }

    // This function concatenates two identifiers.
    pub fn join(&self, other: &Identifier) -> Identifier {
        AsRef::<str>::as_ref(&format!("{}_{}", self.original, other.original)).into()
    }
}

// This function splits a name into words using underscore delimiters and word case.
fn split_words(name: &str) -> Vec<String> {
    let mut snaked = String::new();

    for c in name.chars() {
        if c.is_uppercase() {
            snaked.push('_');
        }
        snaked.push(c);
    }

    snaked
        .split('_')
        .filter_map(|word| {
            if word.is_empty() {
                None
            } else {
                Some(word.to_owned())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{format::CodeStr, identifier::Identifier};
    use std::{
        cmp::Ordering,
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn eq_empty() {
        assert_eq!(Identifier::from(""), Identifier::from(""));
    }

    #[test]
    fn eq_equal() {
        assert_eq!(Identifier::from("hello"), Identifier::from("hello"));
    }

    #[test]
    fn eq_equal_up_to_case() {
        assert_eq!(
            Identifier::from("hello_world"),
            Identifier::from("helloWorld"),
        );
    }

    #[test]
    fn eq_different() {
        assert_ne!(Identifier::from("hello"), Identifier::from("world"));
    }

    #[test]
    fn partial_cmp_empty() {
        assert_eq!(
            Identifier::from("").partial_cmp(&Identifier::from("")),
            Some(Ordering::Equal),
        );
    }

    #[test]
    fn partial_cmp_equal() {
        assert_eq!(
            Identifier::from("hello").partial_cmp(&Identifier::from("hello")),
            Some(Ordering::Equal),
        );
    }

    #[test]
    fn partial_cmp_equal_up_to_case() {
        assert_eq!(
            Identifier::from("hello_world").partial_cmp(&Identifier::from("helloWorld")),
            Some(Ordering::Equal),
        );
    }

    #[test]
    fn partial_cmp_less() {
        assert_eq!(
            Identifier::from("hello").partial_cmp(&Identifier::from("world")),
            Some(Ordering::Less),
        );
    }

    #[test]
    fn partial_cmp_greater() {
        assert_eq!(
            Identifier::from("world").partial_cmp(&Identifier::from("hello")),
            Some(Ordering::Greater),
        );
    }

    #[test]
    fn cmp_empty() {
        assert_eq!(
            Identifier::from("").cmp(&Identifier::from("")),
            Ordering::Equal,
        );
    }

    #[test]
    fn cmp_equal() {
        assert_eq!(
            Identifier::from("hello").cmp(&Identifier::from("hello")),
            Ordering::Equal,
        );
    }

    #[test]
    fn cmp_equal_up_to_case() {
        assert_eq!(
            Identifier::from("hello_world").cmp(&Identifier::from("helloWorld")),
            Ordering::Equal,
        );
    }

    #[test]
    fn cmp_less() {
        assert_eq!(
            Identifier::from("hello").cmp(&Identifier::from("world")),
            Ordering::Less,
        );
    }

    #[test]
    fn cmp_greater() {
        assert_eq!(
            Identifier::from("world").cmp(&Identifier::from("hello")),
            Ordering::Greater,
        );
    }

    #[test]
    fn hash_empty() {
        let mut hasher = DefaultHasher::new();
        Identifier::from("").hash(&mut hasher);
        let actual = hasher.finish();

        let mut hasher = DefaultHasher::new();
        "".hash(&mut hasher);
        let expected = hasher.finish();

        assert_eq!(actual, expected);
    }

    #[test]
    fn hash_snake_case() {
        let mut hasher = DefaultHasher::new();
        Identifier::from("hello_world").hash(&mut hasher);
        let actual = hasher.finish();

        let mut hasher = DefaultHasher::new();
        "hello_world".hash(&mut hasher);
        let expected = hasher.finish();

        assert_eq!(actual, expected);
    }

    #[test]
    fn hash_snake_case_extra_delimiters() {
        let mut hasher = DefaultHasher::new();
        Identifier::from("__hello_world__").hash(&mut hasher);
        let actual = hasher.finish();

        let mut hasher = DefaultHasher::new();
        "hello_world".hash(&mut hasher);
        let expected = hasher.finish();

        assert_eq!(actual, expected);
    }

    #[test]
    fn hash_camel_case() {
        let mut hasher = DefaultHasher::new();
        Identifier::from("helloWorld").hash(&mut hasher);
        let actual = hasher.finish();

        let mut hasher = DefaultHasher::new();
        "hello_world".hash(&mut hasher);
        let expected = hasher.finish();

        assert_eq!(actual, expected);
    }

    #[test]
    fn hash_pascal_case() {
        let mut hasher = DefaultHasher::new();
        Identifier::from("HelloWorld").hash(&mut hasher);
        let actual = hasher.finish();

        let mut hasher = DefaultHasher::new();
        "hello_world".hash(&mut hasher);
        let expected = hasher.finish();

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_empty() {
        assert_eq!(
            Identifier::from(""),
            Identifier {
                original: "".to_owned(),
                snake_case: "".to_owned(),
            },
        );
    }

    #[test]
    fn from_snake_case() {
        assert_eq!(
            Identifier::from("hello_world"),
            Identifier {
                original: "hello_world".to_owned(),
                snake_case: "hello_world".to_owned(),
            },
        );
    }

    #[test]
    fn from_snake_case_extra_delimiters() {
        assert_eq!(
            Identifier::from("__hello_world__"),
            Identifier {
                original: "__hello_world__".to_owned(),
                snake_case: "hello_world".to_owned(),
            },
        );
    }

    #[test]
    fn from_camel_case() {
        assert_eq!(
            Identifier::from("helloWorld"),
            Identifier {
                original: "helloWorld".to_owned(),
                snake_case: "hello_world".to_owned(),
            },
        );
    }

    #[test]
    fn from_pascal_case() {
        assert_eq!(
            Identifier::from("HelloWorld"),
            Identifier {
                original: "HelloWorld".to_owned(),
                snake_case: "hello_world".to_owned(),
            },
        );
    }

    #[test]
    fn code_str_empty() {
        assert_eq!(
            format!("{}", Identifier::from("").code_str()),
            "``".to_owned(),
        );
    }

    #[test]
    fn code_str_snake_case() {
        assert_eq!(
            format!("{}", Identifier::from("hello_world").code_str()),
            "`hello_world`".to_owned(),
        );
    }

    #[test]
    fn code_str_snake_case_extra_delimiters() {
        assert_eq!(
            format!("{}", Identifier::from("__hello_world__").code_str()),
            "`__hello_world__`".to_owned(),
        );
    }

    #[test]
    fn code_str_camel_case() {
        assert_eq!(
            format!("{}", Identifier::from("helloWorld").code_str()),
            "`helloWorld`".to_owned(),
        );
    }

    #[test]
    fn code_str_pascal_case() {
        assert_eq!(
            format!("{}", Identifier::from("HelloWorld").code_str()),
            "`HelloWorld`".to_owned(),
        );
    }

    #[test]
    fn original_empty() {
        assert_eq!(Identifier::from("").original(), "".to_owned());
    }

    #[test]
    fn original_snake_case() {
        assert_eq!(
            Identifier::from("hello_world").original(),
            "hello_world".to_owned(),
        );
    }

    #[test]
    fn original_snake_case_extra_delimiters() {
        assert_eq!(
            Identifier::from("__hello_world__").original(),
            "__hello_world__".to_owned(),
        );
    }

    #[test]
    fn original_camel_case() {
        assert_eq!(
            Identifier::from("helloWorld").original(),
            "helloWorld".to_owned(),
        );
    }

    #[test]
    fn original_pascal_case() {
        assert_eq!(
            Identifier::from("HelloWorld").original(),
            "HelloWorld".to_owned(),
        );
    }

    #[test]
    fn snake_case_empty() {
        assert_eq!(Identifier::from("").snake_case(), "".to_owned());
    }

    #[test]
    fn snake_case_snake_case() {
        assert_eq!(
            Identifier::from("hello_world").snake_case(),
            "hello_world".to_owned(),
        );
    }

    #[test]
    fn snake_case_snake_case_extra_delimiters() {
        assert_eq!(
            Identifier::from("__hello_world__").snake_case(),
            "hello_world".to_owned(),
        );
    }

    #[test]
    fn snake_case_camel_case() {
        assert_eq!(
            Identifier::from("helloWorld").snake_case(),
            "hello_world".to_owned(),
        );
    }

    #[test]
    fn snake_case_pascal_case() {
        assert_eq!(
            Identifier::from("HelloWorld").snake_case(),
            "hello_world".to_owned(),
        );
    }

    #[test]
    fn pascal_case_empty() {
        assert_eq!(Identifier::from("").pascal_case(), "".to_owned());
    }

    #[test]
    fn pascal_case_snake_case() {
        assert_eq!(
            Identifier::from("hello_world").pascal_case(),
            "HelloWorld".to_owned(),
        );
    }

    #[test]
    fn pascal_case_snake_case_extra_delimiters() {
        assert_eq!(
            Identifier::from("__hello_world__").pascal_case(),
            "HelloWorld".to_owned(),
        );
    }

    #[test]
    fn pascal_case_camel_case() {
        assert_eq!(
            Identifier::from("helloWorld").pascal_case(),
            "HelloWorld".to_owned(),
        );
    }

    #[test]
    fn pascal_case_pascal_case() {
        assert_eq!(
            Identifier::from("HelloWorld").pascal_case(),
            "HelloWorld".to_owned(),
        );
    }
}
