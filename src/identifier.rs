use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

// This is a user-provided identifier. To make case-insensitive equality, hashing, etc.
// performance, we pre-compute a case-folded version of the identifier. The main purpose of this
// struct (rather than just using `String`) is to prevent accidental inclusion of identifiers in
// generated code without case conversion.
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

impl Identifier {
    // This function returns the original identifier as provided by the user.
    pub fn original(&self) -> &str {
        &self.original
    }

    // This function returns a `snake_case` version of an identifier.
    pub fn snake_case(&self) -> String {
        self.snake_case.to_owned()
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
    use crate::identifier::Identifier;

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
