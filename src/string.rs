use crate::errors::Result;
use any_ascii::any_ascii;
use regex::Regex;
pub const DEFAULT_SEPARATOR: char = '-';
use heck::AsKebabCase;
use std::string::ToString;
use strip_ansi_escapes::strip as strip_ansi_escapes;

pub fn list_of_trimmed_strings<T: Iterator<Item: std::fmt::Display>>(items: T) -> Vec<String> {
    items
        .map(|part| part.to_string().trim().to_string())
        .filter(|item| item.len() > 0)
        .map(|part| part.to_string().trim().to_string())
        .collect::<Vec<String>>()
}
pub fn slugify_string(haystack: impl std::fmt::Display, separator: char) -> Result<String> {
    let exp = regex_pattern(Some(separator))?;
    let stage0 = haystack.to_string();
    let stage0_bytes = strip_ansi_escapes(&stage0.to_string());
    let stage0_1 = String::from_utf8_lossy(&stage0_bytes);
    let mut stage1_parts = list_of_trimmed_strings(stage0_1.split('\n').into_iter()).join("\n");
    // dbg!(&exp, &stage0, &stage0_1, &stage1_parts);
    for part in ["\t", "\\n", "\n"] {
        // dbg!(&part, &stage1_parts);
        stage1_parts = list_of_trimmed_strings(stage1_parts.split(part).into_iter()).join("\n");
    }
    let stage1 = stage1_parts.to_string();
    let regex_pattern = string_pattern(None);
    let regex = Regex::new(&regex_pattern)?;
    let stage2 = regex.replace_all(&stage1.to_string(), "-").to_string();
    let stage3 = if separator != '-' {
        stage2
            .replace("-", &separator.to_string())
            .trim_matches('-')
            .to_string()
    } else {
        stage2.clone()
    };

    let stage4 = exp
        .replace_all(&stage3, separator.to_string())
        .to_string()
        .as_str()
        .to_string();
    let stage5 = stage4.trim_matches(separator).to_lowercase().to_string();
    // dbg!(&stage1, &stage2, &stage3, &stage4, &stage5);
    Ok(stage5)
}

pub fn string_pattern(separator: Option<char>) -> String {
    match separator {
        Some('_' | '.' | DEFAULT_SEPARATOR) | None => format!("[^a-zA-Z0-9_.-]+"),
        Some(c) => format!("[^a-zA-Z0-9_.{}-]+", c),
    }
}
pub fn regex_pattern(separator: Option<char>) -> Result<Regex> {
    Ok(regex::Regex::new(&string_pattern(separator))?)
}

#[cfg(test)]
mod string_pattern_tests {
    use crate::*;

    #[test]
    fn test_separator_none_underscore_dash() {
        assert_eq!(string_pattern(None), "[^a-zA-Z0-9_.-]+");
        assert_eq!(string_pattern(Some('_')), "[^a-zA-Z0-9_.-]+");
        assert_eq!(string_pattern(Some('-')), "[^a-zA-Z0-9_.-]+");
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
        assert_eq!(string_pattern(None), "[^a-zA-Z0-9_.-]+");
        assert_eq!(string_pattern(Some('_')), "[^a-zA-Z0-9_.-]+");
        assert_eq!(string_pattern(Some('-')), "[^a-zA-Z0-9_.-]+");
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
    // use debug_et_diagnostics::step;

    #[test]
    fn test_slugify_filename() -> Result<()>{
        assert_slugify_string!("\"\n\nwindows-7-preactivated-january-2021-filecr.txt\"", "windows-7-preactivated-january-2021-filecr.txt");
        Ok(())
    }
    #[test]
    fn test_slugify_string() -> Result<()> {
        assert_slugify_string_with_separator!(" Foo Baz ", '-', "foo-baz");
        assert_slugify_string_with_separator!(" Foo Baz ", '_', "foo_baz");
        Ok(())
    }

    #[test]
    fn test_unicode_data_cyrilic_letters() -> Result<()> {
        assert_slugify_string_with_separator!("ÐÐµ, ÑÑÐŸ ÑÐ°Ð·Ð±ÑÐŽÐžÐ» Ð²Ð°Ñ. Ð¯ Ð¿ÑÐŸÑÑÐŸ ÑÐ»ÐžÑÐºÐŸÐŒ Ð²ÐŸÐ·Ð±ÑÐ¶ÐŽÐµÐœ í Ÿíµµ", '-', "d-du-nndy-n-ddeg-d-d-ndz-dz-d-d2-ddeg-n-d-d-ndynndy-nd-dz-n-do-dyd-oe-d2dyd-d-ndpdz-du-doe-i-yiuu");
        Ok(())
    }

    #[macro_export]
    macro_rules! assert_slugify_string_with_separator {
        ($haystack:expr, $separator:literal, $expected_to_be_slugified:expr) => {{
            // use debug_et_diagnostics::step;
            let left = $haystack.to_string();
            let right = $expected_to_be_slugified.to_string();
            let separator = $separator.clone();
            let from = slugify_string(left.to_string(), $separator)?;
            let to = right.to_string();
            // debug_et_diagnostics::step!(format!(
            //     "expect slugify_string({left:#?}) to equal {right:#?}"
            // ));

            assert_eq!(
                from, to,
                "expected slugify_string({left:#?}, {separator:#?})? to equal {right:#?}"
            );
        }};
    }
    #[macro_export]
    macro_rules! assert_slugify_string {
        ($haystack:expr, $expected_to_be_slugified:expr) => {{
            assert_slugify_string_with_separator!($haystack, '_', $expected_to_be_slugified);
        }};
    }
}
