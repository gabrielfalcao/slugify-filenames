use crate::cli::parameters::SlugifyParameters;
use crate::errors::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "slugify-string command-line utility"
)]
pub struct SlugifyString {
    #[arg()]
    text: Vec<String>,

    #[command(flatten)]
    parameters: SlugifyParameters,
}
impl SlugifyString {
    pub fn execute(args: Vec<String>) -> Result<()> {
        let cli = SlugifyString::parse_from(args);
        if !cli.text.is_empty() {
            println!("{}", cli.parameters.slugify_string(cli.text.join(" "),)?);
        }
        Ok(())
    }
}
