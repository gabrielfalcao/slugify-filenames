use std::io::{Write, stdout};
use crate::{argv_fallback_to_stdin_lines, Result, SlugifyParameters};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "slugify-string command-line utility"
)]
pub struct SlugifyString {
    #[arg(value_parser = argv_fallback_to_stdin_lines,  default_value ="")]
    text: Vec<String>,

    #[command(flatten)]
    parameters: SlugifyParameters,
}
impl SlugifyString {
    // pub fn fields(self) -> Result<Vec<String>> {
    //     let text = self.text.clone();
    //     let fields = match iocore::env::var("IFS")
    //         .map(|separators| separators.chars().collect::<Vec<char>>())
    //     {
    //         Ok(separators) => text
    //             .split(|c| separators.contains(&c))
    //             .map(|field| field.to_string())
    //             .collect::<Vec<String>>(),
    //         Err(_) => text
    //             .split_whitespace()
    //             .map(|field| field.to_string())
    //             .collect::<Vec<String>>(),
    //     };
    //     Ok(fields)
    // }
    pub fn lines(self) -> Result<Vec<String>> {
        Ok(self.text.clone())
    }
    pub fn process_lines(self) -> Result<Vec<String>> {
        let lines = self.clone().lines()?;
        let mut result = Vec::<String>::new();
        for line in lines {
            result.push(self.parameters.slugify_string(&line)?);
        }
        Ok(result)
    }
    pub fn process_input(&self) -> Result<()> {
        let result_lines = self.clone().process_lines()?;
        let params = self.parameters.clone();
        let mut output = stdout().lock();
        if params.no_join {
            for line in result_lines.into_iter().map(|line| format!("{line}\n")) {
                output.write(&line.as_bytes())?;
                output.flush()?;
            }
        } else {
            let result = format!(
                "{}\n",
                result_lines.join(&params.non_option_separator().to_string())
            );
            output.write(result.as_bytes())?;
            output.flush()?;
        }
        Ok(())
    }
    pub fn execute(args: Vec<String>) -> Result<()> {
        let cli = &SlugifyString::parse_from(args);
        cli.process_input()?;
        Ok(())
    }
}
