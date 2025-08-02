use crate::errors::Result;
use regex::Regex;

pub const DEFAULT_SEPARATOR: char = '-';

pub fn slugify_string(
    haystack: impl std::fmt::Display,
    separator: char,
    trim_matches: bool,
    lowercase: bool,
) -> Result<String> {
    let exp = regex_pattern(Some(separator))?;
    let haystack = haystack.to_string();
    let result = exp
        .replace_all(&haystack, separator.to_string())
        .to_string()
        .as_str()
        .to_string();
    let result = if trim_matches {
        result.trim_matches(separator).to_string()
    } else {
        result
    };
    let result = if lowercase {
        result.to_lowercase()
    } else {
        result
    };
    let exp = separator_regex_pattern(Some(separator))?;
    let result = exp.replace_all(&result, separator.to_string())
        .to_string()
        .as_str()
        .to_string();

    Ok(result)
}

pub fn string_pattern(separator: Option<char>) -> String {
    match separator {
        Some('_' | DEFAULT_SEPARATOR) | None => format!("[^a-zA-Z0-9_-]+"),
        Some(c) => format!("[^a-zA-Z0-9_{}-]+", c),
    }
}
pub fn regex_pattern(separator: Option<char>) -> Result<Regex> {
    Ok(regex::Regex::new(&string_pattern(separator))?)
}

pub fn separator_string_pattern(separator: Option<char>) -> String {
    match separator {
        Some('_' | DEFAULT_SEPARATOR) | None => format!("[-]+"),
        Some(c) => format!("[_{}-]+", c),
    }
}
pub fn separator_regex_pattern(separator: Option<char>) -> Result<Regex> {
    Ok(regex::Regex::new(&separator_string_pattern(separator))?)
}
#[cfg(test)]
mod string_pattern_tests {
    use crate::*;

    #[test]
    fn test_separator_none_underscore_dash() {
        assert_eq!(string_pattern(None), "[^a-zA-Z0-9_-]+");
        assert_eq!(string_pattern(Some('_')), "[^a-zA-Z0-9_-]+");
        assert_eq!(string_pattern(Some('-')), "[^a-zA-Z0-9_-]+");
    }
    #[test]
    fn test_separator_dot() {
        assert_eq!(string_pattern(Some('.')), "[^a-zA-Z0-9_.-]+");
    }
}

#[cfg(test)]
mod regex_pattern_tests {
    use crate::*;

    #[test]
    fn test_separator_none_underscore_dash() -> Result<()> {
        assert_eq!(string_pattern(None), "[^a-zA-Z0-9_-]+");
        assert_eq!(string_pattern(Some('_')), "[^a-zA-Z0-9_-]+");
        assert_eq!(string_pattern(Some('-')), "[^a-zA-Z0-9_-]+");
        Ok(())
    }
    #[test]
    fn test_separator_dot() -> Result<()> {
        assert_eq!(string_pattern(Some('.')), "[^a-zA-Z0-9_.-]+");
        Ok(())
    }
}

#[cfg(test)]
mod slugify_string_tests {
    use crate::*;

    #[test]
    fn test_slugify_string() -> Result<()> {
        assert_eq!(slugify_string(" Foo Baz ", '-', true, false)?, "Foo-Baz");
        assert_eq!(slugify_string(" Foo Baz ", '-', true, false)?, "Foo-Baz");
        assert_eq!(slugify_string(" Foo Baz ", '_', true, false)?, "Foo_Baz");
        assert_eq!(slugify_string(" Foo Baz ", '-', false, false)?, "-Foo-Baz-");
        assert_eq!(slugify_string(" Foo Baz ", '_', false, false)?, "_Foo_Baz_");

        assert_eq!(slugify_string(" Foo Baz ", '-', true, true)?, "foo-baz");
        assert_eq!(slugify_string(" Foo Baz ", '-', true, true)?, "foo-baz");
        assert_eq!(slugify_string(" Foo Baz ", '_', true, true)?, "foo_baz");
        assert_eq!(slugify_string(" Foo Baz ", '-', false, true)?, "-foo-baz-");
        assert_eq!(slugify_string(" Foo Baz ", '_', false, true)?, "_foo_baz_");
        Ok(())
    }
}
