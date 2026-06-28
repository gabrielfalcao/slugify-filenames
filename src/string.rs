use crate::errors::Result;
use any_ascii::any_ascii;
use regex::Regex;
use std::string::ToString;
use strip_ansi_escapes::strip as strip_ansi_escapes;

pub const DEFAULT_SEPARATOR: char = '-';
pub const STRING_PATTERN: &'static str = r"[^a-zA-Z0-9_.-]+";
pub const SPECIAL_PATTERN_CHARS: [char; 3] = ['_', '.', '-'];

pub fn list_of_trimmed_strings<T: Iterator<Item: std::fmt::Display>>(items: T) -> Vec<String> {
    items
        .map(|part| part.to_string().trim().to_string())
        .filter(|item| item.len() > 0)
        .map(|part| part.to_string().trim().to_string())
        .collect::<Vec<String>>()
}
pub fn slugify_string(haystack: impl std::fmt::Display) -> Result<String> {
    let stage0 = haystack.to_string();
    let stage0_bytes = strip_ansi_escapes(&stage0.to_string());
    let stage0_1 = String::from_utf8_lossy(&stage0_bytes);
    let mut stage1_parts = list_of_trimmed_strings(stage0_1.split('\n').into_iter()).join("\n");
    for part in ["\t", "\\n", "\n"] {
        stage1_parts = list_of_trimmed_strings(stage1_parts.split(part).into_iter()).join("\n");
    }
    let stage1 = any_ascii(&stage1_parts);
    let main_regex = Regex::new(&STRING_PATTERN)?;
    let stage2 = main_regex.replace_all(&stage1, r"-").to_string();
    let mut stage3 = stage2.to_lowercase().to_string();
    for c in SPECIAL_PATTERN_CHARS.iter().map(|c| *c) {
        let dupe_pattern = format!("[{c}][c]+");
        let re = Regex::new(&dupe_pattern)?;

        stage3 = stage3.trim_start_matches(c).to_string();
        stage3 = stage3.trim_end_matches(c).to_string();
        stage3 = re.replace_all(&stage3, &c.to_string()).to_string();
    }
    Ok(stage3)
}

#[cfg(test)]
mod slugify_string_tests {
    use crate::{assert_slugify_string, slugify_string, Error, Result};

    #[test]
    fn test_slugify_filename() -> Result<()> {
        assert_slugify_string!(
            "\"\n\nwindows-7-preactivated-january-2021-filecr.txt\"",
            "windows-7-preactivated-january-2021-filecr.txt"
        );
        Ok(())
    }
    #[test]
    fn test_slugify_string() -> Result<()> {
        assert_slugify_string!("  Foo  Baz  ", "foo-baz");
        assert_slugify_string!("  Foo  Baz  ", "foo-baz");
        Ok(())
    }

    #[test]
    fn test_unicode_data_cyrilic_letters() -> Result<()> {
        assert_slugify_string!("ÐÐµ, ÑÑÐŸ ÑÐ°Ð·Ð±ÑÐŽÐžÐ» Ð²Ð°Ñ. Ð¯ Ð¿ÑÐŸÑÑÐŸ ÑÐ»ÐžÑÐºÐŸÐŒ Ð²ÐŸÐ·Ð±ÑÐ¶ÐŽÐµÐœ í Ÿíµµ", "ddu-nndy-nddegd-d--ndzdzd-d2ddegn.-d--d-ndynndy-nd-dzndodydoe-d2dyd-d--ndpdzdudoe-i-yiuu");
        Ok(())
    }

    #[macro_export]
    macro_rules! assert_slugify_string {
        ($haystack:expr, $expected_to_be_slugified:expr) => {{
            assert_eq!(slugify_string($haystack)?, $expected_to_be_slugified);
        }};
    }
}
