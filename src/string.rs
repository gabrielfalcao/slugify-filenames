use crate::errors::Result;
use any_ascii::any_ascii;
use regex::Regex;
use std::string::ToString;
use std::sync::LazyLock;
use strip_ansi_escapes::strip as strip_ansi_escapes;

pub const DEFAULT_SEPARATOR: char = '-';
pub static STRING_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^a-zA-Z0-9_.-]+").expect("STRING_REGEX"));
pub static UNNEEDED_UNIQUEFY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[0-9]+[.](?<extension>[a-zA-Z0-9]+)$").expect("UNNEEDED_UNIQUEFY_REGEX")
});
pub static DUPE_SEPARATOR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[-][-]+").expect("STRING_REGEX"));

pub const SPECIAL_PATTERN_CHARS: [char; 3] = ['_', '.', '-'];

pub fn list_of_trimmed_strings<T: Iterator<Item: std::fmt::Display>>(items: T) -> Vec<String> {
    items
        .map(|part| part.to_string().trim().to_string())
        .filter(|item| item.len() > 0)
        .map(|part| part.to_string().trim().to_string())
        .collect::<Vec<String>>()
}

/// `slugify_string` is the core function in this package.
///
/// *Example*
///
/// ```
/// use slugify_filenames::{slugify_string, Error};
///
/// // passing `false` in the second argument to prevent downcasing the string
/// let result = slugify_string("Imagine Thís STRING, àscii Safê and Filename-sáfè", false)?;
/// assert_eq!(result, "Imagine-This-STRING-ascii-Safe-and-Filename-safe");
///
/// // passing `true` in the second argument to downcase the string
/// let result = slugify_string("Generated_FILE_ymf1a3ymf1a3ymf1.png.0.gz", true)?;
/// assert_eq!(result, "generated_file_ymf1a3ymf1a3ymf1.png.gz");
/// # Ok::<(), Error>(())
/// ```
///
pub fn slugify_string<T: std::string::ToString>(haystack: T, downcase: bool) -> Result<String> {
    let stage0 = haystack.to_string();
    let stage0_bytes = strip_ansi_escapes(&stage0.to_string());
    let stage0_1 = String::from_utf8_lossy(&stage0_bytes);
    let mut stage1_parts = list_of_trimmed_strings(stage0_1.split('\n').into_iter()).join("\n");
    for part in ["\t", "\\n", "\n"] {
        stage1_parts = list_of_trimmed_strings(stage1_parts.split(part).into_iter()).join("\n");
    }
    let stage1 = any_ascii(&stage1_parts);
    let stage2 = STRING_REGEX.replace_all(&stage1, r"-").to_string();
    let stage3 = if let Some(caps) = UNNEEDED_UNIQUEFY_REGEX.captures(&stage2) {
        let extension = caps
            .get(1)
            .map(|g| g.as_str().to_string())
            .unwrap_or_default();
        UNNEEDED_UNIQUEFY_REGEX
            .replace_all(&stage2, &extension)
            .to_string()
    } else {
        stage2.to_string()
    };
    let mut stage4 = stage3.to_string();
    for c in SPECIAL_PATTERN_CHARS.iter().map(|c| *c) {
        let dupe_pattern = format!("[{c}][c]+");
        let re = Regex::new(&dupe_pattern)?;

        stage4 = re.replace_all(&stage4, &c.to_string()).to_string();
        stage4 = stage4.trim_start_matches(c).to_string();
        stage4 = stage4.trim_end_matches(c).to_string();
    }
    let stage5 = DUPE_SEPARATOR_REGEX.replace_all(&stage4, "-").to_string();
    let stage6 = if downcase {
        stage5.to_lowercase()
    } else {
        stage5.clone()
    };
    Ok(stage6)
}

#[cfg(test)]
mod slugify_string_tests {
    use crate::{assert_slugify_string, list_of_trimmed_strings, slugify_string, Result};

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
        assert_slugify_string!("  Foo  Baz  ", "Foo-Baz");
        assert_slugify_string!(downcase "  Foo  Baz  ", "foo-baz");
        Ok(())
    }

    #[test]
    fn test_unicode_data_cyrilic_letters() -> Result<()> {
        assert_slugify_string!(downcase "ÐÐµ, ÑÑÐŸ ÑÐ°Ð·Ð±ÑÐŽÐžÐ» Ð²Ð°Ñ. Ð¯ Ð¿ÑÐŸÑÑÐŸ ÑÐ»ÐžÑÐºÐŸÐŒ Ð²ÐŸÐ·Ð±ÑÐ¶ÐŽÐµÐœ í Ÿíµµ", "ddu-nndy-nddegd-d-ndzdzd-d2ddegn.-d-d-ndynndy-nd-dzndodydoe-d2dyd-d-ndpdzdudoe-i-yiuu");
        assert_slugify_string!(
            "ÐÐµ, ÑÑÐŸ ÑÐ°Ð·Ð±ÑÐŽÐžÐ» Ð²Ð°Ñ. Ð¯ Ð¿ÑÐŸÑÑÐŸ ÑÐ»ÐžÑÐºÐŸÐŒ Ð²ÐŸÐ·Ð±ÑÐ¶ÐŽÐµÐœ í Ÿíµµ",
            "DDu-NNDY-NDdegD-D-NDZDzD-D2DdegN.-D-D-NDYNNDY-ND-DzNDoDYDOe-D2DYD-D-NDPDZDuDoe-i-Yiuu"
        );
        Ok(())
    }

    #[test]
    fn test_list_of_trimmed_strings() -> Result<()> {
        assert_eq!(
            list_of_trimmed_strings([" this is an ", "    string    input    "].iter()),
            vec!["this is an".to_string(), "string    input".to_string()]
        );
        Ok(())
    }
    #[macro_export]
    macro_rules! assert_slugify_string {
        (downcase $haystack:expr, $expected_to_be_slugified:expr) => {{
            assert_eq!(slugify_string($haystack, true)?, $expected_to_be_slugified);
        }};

        ($haystack:expr, $expected_to_be_slugified:expr) => {{
            assert_eq!(slugify_string($haystack, false)?, $expected_to_be_slugified);
        }};
    }
}
