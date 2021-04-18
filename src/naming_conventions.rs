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

// This function converts a word to `Uppercase`.
fn uppercase(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

// This function converts a name to `snake_case`.
pub fn snake_case(name: &str) -> String {
    split_words(name)
        .iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

// This function converts a name to `PascalCase`.
pub fn pascal_case(name: &str) -> String {
    split_words(name)
        .iter()
        .map(|word| uppercase(word))
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use crate::naming_conventions::{pascal_case, snake_case, split_words, uppercase};

    #[test]
    fn split_words_empty() {
        assert_eq!(split_words(""), vec![] as Vec<&str>);
    }

    #[test]
    fn split_words_snake_case() {
        assert_eq!(split_words("hello_world"), vec!["hello", "world"]);
    }

    #[test]
    fn split_words_snake_case_extra_delimiters() {
        assert_eq!(split_words("__hello_world__"), vec!["hello", "world"]);
    }

    #[test]
    fn split_words_camel_case() {
        assert_eq!(split_words("helloWorld"), vec!["hello", "World"]);
    }

    #[test]
    fn split_words_pascal_case() {
        assert_eq!(split_words("HelloWorld"), vec!["Hello", "World"]);
    }

    #[test]
    fn uppercase_empty() {
        assert_eq!(uppercase(""), "".to_owned());
    }

    #[test]
    fn uppercase_lowercase() {
        assert_eq!(uppercase("hello"), "Hello".to_owned());
    }

    #[test]
    fn uppercase_uppercase() {
        assert_eq!(uppercase("Hello"), "Hello".to_owned());
    }

    #[test]
    fn snake_case_empty() {
        assert_eq!(snake_case(""), "".to_owned());
    }

    #[test]
    fn snake_case_snake_case() {
        assert_eq!(snake_case("hello_world"), "hello_world".to_owned());
    }

    #[test]
    fn snake_case_snake_case_extra_delimiters() {
        assert_eq!(snake_case("__hello_world__"), "hello_world".to_owned());
    }

    #[test]
    fn snake_case_camel_case() {
        assert_eq!(snake_case("helloWorld"), "hello_world".to_owned());
    }

    #[test]
    fn snake_case_pascal_case() {
        assert_eq!(snake_case("HelloWorld"), "hello_world".to_owned());
    }

    #[test]
    fn pascal_case_empty() {
        assert_eq!(pascal_case(""), "".to_owned());
    }

    #[test]
    fn pascal_case_snake_case() {
        assert_eq!(pascal_case("hello_world"), "HelloWorld".to_owned());
    }

    #[test]
    fn pascal_case_snake_case_extra_delimiters() {
        assert_eq!(pascal_case("__hello_world__"), "HelloWorld".to_owned());
    }

    #[test]
    fn pascal_case_camel_case() {
        assert_eq!(pascal_case("helloWorld"), "HelloWorld".to_owned());
    }

    #[test]
    fn pascal_case_pascal_case() {
        assert_eq!(pascal_case("HelloWorld"), "HelloWorld".to_owned());
    }
}
