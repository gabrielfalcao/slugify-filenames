use crate::errors::{Error, Result};
// use heck::AsKebabCase;
use iocore::Path;
use is_terminal::IsTerminal;
use regex::Captures;
use regex::Regex;
use sanitation::SString;
use std::convert::AsRef;
use std::io::BufRead;
use std::string::ToString;
use strip_ansi_escapes::strip as strip_ansi_escapes;

pub const DEFAULT_SEPARATOR: char = '-';

pub fn get_stdin_lines() -> Result<Vec<String>> {
    let handle = std::io::stdin().lock();
    let mut result = Vec::<String>::new();
    if !handle.is_terminal() {
        for line in handle.lines() {
            result.push(line?);
        }
        Ok(result)
    } else {
        Err(Error::IOError(format!("stdin is a tty")))
    }
}

pub fn get_stdin_text() -> Result<String> {
    get_stdin_lines().map(|lines| lines.join("\n").to_string())
}
pub fn undupe<T: ToString>(name: T) -> String {
    let haystack = name.to_string();
    // let regexes = [".", "/", "_", "-"].map(|c|format!("([{c}])")).map(|class|Regex::new(class.as_str()).unwrap())

    let regex = Regex::new("([./_-])($1)+").unwrap();
    regex.replace(&haystack, "$1").to_string()
}
pub fn name_splitext<T: ToString>(name: T) -> Option<(String, String)> {
    let haystack = name.to_string();
    let regex = Regex::new(
        "^(?<path>(.*?)([^.]+))(?<sub_extensions>[.][a-zA-Z0-9]+)*(?<extension>[.][a-zA-Z0-9]+)?(?<rest>[.a-zA-Z0-9_/-]+?.*?)?$",
    ).unwrap();
    let found = regex.captures(&haystack)?;
    let path = found.name("path").map(|m| m.as_str().to_string())?;
    let sub_extensions = found
        .name("sub_extensions")
        .map(|m| m.as_str().to_string())?;
    let extension = found.name("extension").map(|m| m.as_str().to_string())?;

    let ext = [sub_extensions, extension].join(".").to_string();
    let path = path.to_string();
    Some((undupe(path), undupe(ext)))
}

pub fn slugify_path<T: AsRef<std::path::Path>>(path_ref: T, separator: char) -> Result<Path> {
    let path_str = Path::from(path_ref.as_ref()).to_string();
    let orig_path = Path::from(&path_str);
    let path = orig_path.canonicalize()?;
    let parent = path.parent();
    let path = Path::new(path.name());
    let slugified_filename = match name_splitext(&path.name()) {
        Some((base, extension)) => {
            let slugified_base = slugify_string(&base, separator)?;
            let slugified_extension = slugify_string(&extension, separator)?;
            let slugified_filename = format!("{slugified_base}.{slugified_extension}");

            if base.starts_with(".") {
                format!(".{slugified_filename}")
            } else {
                slugified_filename
            }
        }
        None => {
            let base = path.name();
            let slugified_filename = slugify_string(&base.to_string(), separator)?;

            if base.starts_with(".") {
                format!(".{slugified_filename}")
            } else {
                slugified_filename
            }
        }
    };
    match parent {
        Some(parent) => Ok(parent.join(slugified_filename)),
        None => Ok(Path::new(slugified_filename)),
    }
}

pub fn slugify_string<T: std::fmt::Display>(haystack: T, separator: char) -> Result<String> {
    let exp = regex_pattern(Some(separator))?;
    let stage0 = haystack.to_string();
    let stage0_bytes = strip_ansi_escapes(&stage0);
    let stage1 = SString::new(&stage0_bytes).unchecked_safe();
    let stage2 = deunicode::deunicode_with_tofu(&stage1, &separator.to_string());
    let stage3 = exp
        .replace_all(&stage2, separator.to_string())
        .to_string()
        .as_str()
        .to_string();
    let stage4 = stage3.trim_matches(separator).to_lowercase().to_string();
    let stage5 = if separator != '-' {
        stage4
            .replace("-", &separator.to_string())
            .trim_matches('-')
            .to_string()
    } else {
        stage4.clone().trim_matches('-').to_string()
    };
    Ok(stage5)
}

pub fn string_pattern(separator: Option<char>) -> String {
    match separator {
        Some(DEFAULT_SEPARATOR) | None => format!("[^a-zA-Z0-9.-]+"),
        Some('_') => format!("[^a-zA-Z0-9._-]+"),
        Some(c) => format!("[^a-zA-Z0-9.{c}-]+"),
    }
}
pub fn regex_pattern(separator: Option<char>) -> Result<Regex> {
    Ok(regex::Regex::new(&string_pattern(separator))?)
}

pub fn extension_regex() -> Result<Regex> {
    Ok(Regex::new(
        "^(?<path>(.*?)([^.]+))(?<sub_extensions>[.][a-zA-Z0-9]+)*(?<extension>[.][a-zA-Z0-9]+)?(?<rest>[.a-zA-Z0-9_/-]+?.*?)?$",
    )?)
}
pub fn slugify_path_regex<T: ToString>(path_ref: T) -> Result<Path> {
    let path_str = path_ref.to_string();
    let orig_path = Path::from(&path_str);
    let path = orig_path.absolute()?;
    let parent = path
        .parent()
        .ok_or_else(|| Error::ValueError(format!("path {path_str:#?} should have a parent")))?;
    let path = Path::new(path.name());
    let (regex, caps) = path_extension(&path)?;
    let filename = caps
        .name("path")
        .map(|val| val.as_str().to_string())
        .ok_or_else(|| Error::ValueError(format!("could not match path from {path_str:?}")))?;
    let sub_extensions = caps
        .name("sub_extensions")
        .map(|val| val.as_str().to_string())
        .ok_or_else(|| {
            Error::ValueError(format!("could not match sub-extensions from {path_str:?}"))
        })?;

    let extension = caps
        .name("extension")
        .map(|val| val.as_str().to_string())
        .ok_or_else(|| Error::ValueError(format!("could not match extension from {path_str:?}")))?;

    let result = parent.join(format!("{filename}.{sub_extensions}.{extension}"));
    return Ok(result);
}
pub fn path_extension<T: ToString>(path_str: T) -> Result<(Regex, Captures<'static>)> {
    let regex = extension_regex()?;
    let haystack = path_str.to_string();
    match regex.clone().captures(haystack.clone().leak()) {
        Some(caps) => Ok((regex, caps)),
        None => Err(Error::ArgumentError(format!(
            "cannot match regex {regex:#?} to haystack {haystack:#?}"
        ))),
    }
}

#[cfg(test)]
mod path_extension_regex_tests {
    use crate::*;
    use k9::assert_equal;
    #[test]
    fn test_extract_extension_from_tarball() -> Result<()> {
        assert_equal!(
            slugify_path_regex("dummylib.4-2-0.tar.gz")?.to_string(),
            format!("")
        );
        Ok(())
    }
}

#[cfg(test)]
mod string_pattern_tests {
    use crate::*;

    #[test]
    fn test_separator_none_underscore_dash() {
        assert_eq!(string_pattern(None), "[^a-zA-Z0-9_./-]+");
        assert_eq!(string_pattern(Some('_')), "[^a-zA-Z0-9_./-]+");
        assert_eq!(string_pattern(Some('-')), "[^a-zA-Z0-9_./-]+");
    }
    #[test]
    fn test_separator_dot() {
        assert_eq!(string_pattern(Some('.')), "[^a-zA-Z0-9_./-]+");
    }
}

#[cfg(test)]
mod regex_pattern_tests {
    use crate::*;

    #[test]
    fn test_separator_none_underscore_dash() -> Result<()> {
        assert_eq!(string_pattern(None), "[^a-zA-Z0-9_./-]+");
        assert_eq!(string_pattern(Some('_')), "[^a-zA-Z0-9_./-]+");
        assert_eq!(string_pattern(Some('-')), "[^a-zA-Z0-9_./-]+");
        Ok(())
    }
    #[test]
    fn test_separator_dot() -> Result<()> {
        assert_eq!(string_pattern(Some('.')), "[^a-zA-Z0-9_./-]+");
        Ok(())
    }
}

// #[cfg(test)]
// mod slugify_string_tests {
//     use crate::*;
//     // use debug_et_diagnostics::step;
//     #[test]
//     fn test_slugify_string() -> Result<()> {
//         assert_slugify_string!("Gabriel Falcão", '-', "gabriel-falcao");
//         assert_slugify_string!(" Foo Baz ", '-', "foo-baz");
//         assert_slugify_string!(" Foo Baz ", '_', "foo_baz");
//         Ok(())
//     }
//
//     #[test]
//     fn test_unicode_data_cyrilic_letters() -> Result<()> {
//         assert_slugify_string!("ÐÐµ, ÑÑÐŸ ÑÐ°Ð·Ð±ÑÐŽÐžÐ» Ð²Ð°Ñ. Ð¯ Ð¿ÑÐŸÑÑÐŸ ÑÐ»ÐžÑÐºÐŸÐŒ Ð²ÐŸÐ·Ð±ÑÐ¶ÐŽÐµÐœ í Ÿíµµ", '-', "ddu-nndy-nddegd-d--ndzdzd-d2ddegn.-d--d-ndynndy-nd-dzndodydoe-d2dyd-d--ndpdzdudoe-i-yiuu");
//         Ok(())
//     }
//
//     #[rustfmt::skip]
//     #[test]
//     fn test_transliteration() -> Result<()>{
//         assert_slugify_string!("✓", '-', "ok");
//         assert_slugify_string!("Æneid", '-', "aeneid");
//         assert_slugify_string!("étude", '-', "etude");
//         assert_slugify_string!("北亰", '-', "bei-jing");
//         assert_slugify_string!("北亰city", '-', "bei-jing-city");
//         assert_slugify_string!("北亰 city", '-', "bei-jing-city");
//         assert_slugify_string!("北 亰 — city", '-', "bei-jing-city");
//         assert_slugify_string!("北亰 city ", '-', "bei-jing-city");
//         assert_slugify_string!("ᔕᓇᓇ", '-', "shanana");
//         assert_slugify_string!("ᏔᎵᏆ", '-', "taliqua");
//         assert_slugify_string!("ܦܛܽܐܺ", '-', "ptu-i");
//         assert_slugify_string!("अभिजीत", '-', "abhijiit");
//         assert_slugify_string!("অভিজীত", '-', "abhijiit");
//         assert_slugify_string!("അഭിജീത", '-', "abhijiit");
//         assert_slugify_string!("മലയാലമ്", '-', "mlyaalm");
//         assert_slugify_string!("げんまい茶", '-', "genmai-cha");
//         assert_slugify_string!("🦄☣", '-', "unicorn-biohazard");
//         assert_slugify_string!("🦄 ☣", '-', "unicorn-biohazard");
//         assert_slugify_string!("🦄 ☣", '-', "unicorn-biohazard");
//         assert_slugify_string!(" spaces ", '-', "spaces");
//         assert_slugify_string!("  two  spaces  ", '-', "two-spaces");
//         assert_slugify_string!(&[std::char::from_u32(61849).unwrap()].iter().collect::<String>(), '-', "");
//         assert_slugify_string!(&[std::char::from_u32(61849).unwrap()].iter().collect::<String>(), '-', "");
//         assert_slugify_string!("\u{2713} [x]", '-', "ok-x");
//         assert_slugify_string!("技术", '-', "ji-shu");
//         assert_slugify_string!("评价", '-', "ping-jia");
//         assert_slugify_string!("旅游", '-', "lv-you");
//         assert_slugify_string!("旅游", '-', "lv-you");
//         Ok(())
//     }
//
//     #[macro_export]
//     macro_rules! assert_slugify_string {
//         ($haystack:expr, $separator:literal, $expected_to_be_slugified:expr) => {{
//             // use debug_et_diagnostics::step;
//             let left = $haystack.to_string();
//             let right = $expected_to_be_slugified.to_string();
//             let separator = $separator.clone();
//             let from = slugify_string(left.to_string(), $separator)?;
//             let to = right.to_string();
//             // debug_et_diagnostics::step!(format!(
//             //     "expect slugify_string({left:#?}) to equal {right:#?}"
//             // ));
//
//             assert_eq!(
//                 from, to,
//                 "expected slugify_string({left:#?}, {separator:#?})? to equal {right:#?}"
//             );
//         }};
//     }
// }
