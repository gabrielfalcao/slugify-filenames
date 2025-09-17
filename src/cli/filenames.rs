use crate::cli::parameters::SlugifyParameters;
use crate::errors::*;

use clap::Parser;
use iocore::Path;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "slugify-filenames command-line utility"
)]
pub struct SlugifyFilenames {
    #[arg()]
    paths: Vec<Path>,

    #[arg(short, long)]
    max_depth: Option<usize>,

    #[command(flatten)]
    parameters: SlugifyParameters,

    #[arg(short, long)]
    quiet: bool,

    #[arg(short = 'S', long, help = "path to .slugifyignore file")]
    slugify_ignore: Option<Path>,
}
impl SlugifyFilenames {
    pub fn paths(&self) -> Vec<Path> {
        if self.paths.len() > 0 {
            self.paths.clone()
        } else {
            Path::cwd().list().unwrap_or_default()
        }
    }
    pub fn println(&self, string: impl std::fmt::Display) {
        if !self.quiet {
            println!("{string}");
        }
    }
    pub fn slugify_ignore_path(&self) -> Result<Path> {
        if let Some(path) = &self.slugify_ignore {
            if !path.exists() {
                return Err(Error::IOError(format!(
                    "the provided slugify ignore file does not exist: {path:#?}"
                )));
            }
        }
        Ok(self
            .slugify_ignore
            .clone()
            .unwrap_or_else(|| Path::new(".slugifyignore"))
            .try_canonicalize())
    }
    pub fn slugify_ignore_lines(&self) -> Result<Vec<String>> {
        let path = self.slugify_ignore_path()?;
        if path.is_file() {
            if !self.quiet {
                eprintln!("trying to read slugifyignores file {path}");
            }
            Ok(path
                .read()?
                .lines()
                .map(|line| line.trim().to_string())
                .collect())
        } else {
            Ok(Vec::new())
        }
    }
    pub fn should_ignore(&self, lines: &Vec<String>, path: &Path) -> Result<bool> {
        if lines.is_empty() {
            Ok(false)
        } else {
            Ok(lines.iter().any(|line| {
                let line = line.to_string();
                line == path.name()
                    || line == path.relative_to_cwd().to_string()
                    || line == path.to_string()
            }))
        }
    }
    pub fn slugify_file_path(&self, path: &Path) -> Result<Path> {
        let path = path.canonicalize()?;
        let (name, extension) = if path.is_file() {
            path.split_extension()
        } else {
            (path.name(), None)
        };

        let new_name = self.parameters.slugify_string(&name)?;
        let new_extension = match extension.clone() {
            Some(extension) => Some(self.parameters.slugify_string(extension)?),
            None => None,
        };

        let mut count = 0;
        let new_filename = Path::join_extension(&new_name, new_extension.clone());
        let mut new_path = path.with_filename(&new_filename);
        while path.name() != new_path.name() && new_path.exists() {
            let new_filename = Path::join_extension(format!("{new_name}.{count}"), new_extension.clone());
            new_path = path.with_filename(&new_filename);
            count+=1;
        }
        if path.name() != new_path.name() {
            let new_path = match path.rename(&new_path, true) {
                Ok(new_path) => new_path,
                Err(error) => return Err(Error::IOError(format!("{}", error))),
            };
            self.println(format!("{path} => {new_path}"));
            Ok(new_path.canonicalize()?)
        } else {
            Ok(path.canonicalize()?)
        }
    }
    pub fn slugify_path(&self, path: &Path) -> Result<()> {
        let new_path = self.slugify_file_path(path)?;
        if new_path.is_dir() {
            for sub_path in new_path.list()? {
                // let name = sub_path.name();
                // let slug = self.parameters.slugify_string(&name)?;
                // eprintln!("{name} => {slug}");
                self.slugify_path(&sub_path)?;
            }
        }
        // self.slugify_file_path(path)?;
        Ok(())
    }
    pub fn execute(args: Vec<String>) -> Result<()> {
        let cli = SlugifyFilenames::parse_from(args);
        let ignores = cli.slugify_ignore_lines()?;
        for old_path in cli.paths().iter() {
            let old_path = old_path.canonicalize()?;
            if cli.should_ignore(&ignores, &old_path)? {
                eprintln!("ignoring {old_path}");
                return Ok(());
            }
            cli.slugify_path(&old_path)?;
        }
        Ok(())
    }
}
