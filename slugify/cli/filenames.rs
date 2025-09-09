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

    #[arg(short, long)]
    force: bool,

    #[arg(short, long)]
    dry_run: bool,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short = 'I', long, help = "path to .slugifyignore file")]
    slugify_ignore: Option<Path>,
}
impl SlugifyFilenames {
    pub fn println(&self, string: impl std::fmt::Display) {
        if !self.quiet {
            println!("{}", string)
        }
    }
    pub fn paths(&self) -> Vec<Path> {
        if self.paths.is_empty() {
            let cwd = Path::cwd().try_canonicalize();
            self.println(format!("no paths provided, assuming {}", cwd.abbreviate()));
            cwd.list().unwrap_or_default()
        } else {
            self.paths.clone()
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
            self.println(format!("trying to read {path}"));
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
        let (name, extension) = path.split_extension();
        let new_name = self.parameters.slugify_string(&name)?;
        let new_extension = match extension.clone() {
            Some(extension) => Some(self.parameters.slugify_string(extension)?),
            None => None,
        };

        let new_filename = Path::join_extension(new_name, new_extension);
        let new_path = path.with_filename(&new_filename);
        if *path != new_path {
            if self.dry_run {
                self.println(format!("would rename {path} to {new_path}"));
                return Ok(new_path);
            }
            if path.is_dir() && new_path.is_dir() && self.force {
                return Ok(new_path.try_canonicalize());
            } else if !self.force {
                return Err(Error::IOError(format!(
                    "{new_path} already exists, use --force to overwrite"
                )));
            }
            let new_path = match path.rename(&new_path, true) {
                Ok(new_path) => new_path,
                Err(error) => return Err(Error::IOError(format!("{}", error))),
            };
            self.println(format!("{} -> {}", path, &new_path));
            Ok(new_path.try_canonicalize())
        } else {
            Ok(path.try_canonicalize())
        }
    }
    pub fn slugify_path(&self, path: &Path) -> Result<()> {
        let new_path = self.slugify_file_path(path)?;
        if self.recursive && new_path.is_dir() {
            for sub_path in new_path.list()? {
                self.slugify_path(&sub_path)?;
            }
        }
        Ok(())
    }
    pub fn execute(args: Vec<String>) -> Result<()> {
        let cli = SlugifyFilenames::parse_from(args);
        let ignores = cli.slugify_ignore_lines()?;
        let paths = cli.paths();
        let total_paths = paths.len();
        let target_paths = paths
            .clone()
            .into_iter()
            .filter(|old_path| cli.should_ignore(&ignores, &old_path).unwrap_or_default())
            .collect::<Vec<Path>>();
        let total_filtered = target_paths.len();
        if total_filtered == 0 {
            if total_paths > 0 {
                cli.println(format!(
                    "total paths is {total_paths} but all have been ignored: "
                ));
                for path in paths.iter() {
                    let path = path.relative_to_cwd();
                    cli.println(format!("    {path}"));
                }
            } else {
                cli.println(format!("no paths to slugify"));
            }
            return Ok(());
        }
        for old_path in target_paths {
            cli.slugify_path(&old_path)?;
        }
        Ok(())
    }
}
