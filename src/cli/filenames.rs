use crate::cli::parameters::SlugifyParameters;
use crate::cli::verbosity::Verbosity;

pub use crate::errors::{Error, Result};

use clap::{ArgAction, Parser};
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

    #[arg(short, long, help = "decrease verbosity", conflicts_with_all=["dry_run"], action = ArgAction::Count)]
    quiet: u8,

    #[arg(short, long, help = "increase verbosity", action = ArgAction::Count)]
    verbose: u8,

    #[arg(long, default_value = "info", conflicts_with_all=["quiet", "verbose"], help = "set verbosity explicitly rather than by increments or decrements (.i.e.: `--verbose' or `--quiet')")]
    verbosity: Verbosity,

    #[arg(short, long)]
    force: bool,

    #[arg(long)]
    debug: bool,

    #[arg(short, long)]
    dry_run: bool,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short = 'I', long, help = "path to .slugifyignore file")]
    slugify_ignore: Option<Path>,
}
impl SlugifyFilenames {
    pub fn actual_verbosity(&self) -> Verbosity {
        // dbg!(&self.verbosity, &self.quiet, &self.verbose);
        Verbosity::from(self.verbosity.level() - self.quiet + self.verbose)
    }
    pub fn verbosity_matches(&self, verbosity: Verbosity) -> bool {
        // dbg!(&self.actual_verbosity(), &verbosity);
        verbosity.level() <= self.actual_verbosity().level()
    }
    pub fn println(&self, string: impl std::fmt::Display, verbosity: Verbosity) {
        if self.verbosity_matches(verbosity) {
            println!("{}", string)
        }
    }
    pub fn eprintln(&self, string: impl std::fmt::Display, verbosity: Verbosity) {
        if self.verbosity_matches(verbosity) {
            eprintln!("{}", string)
        }
    }
    pub fn paths(&self) -> Vec<Path> {
        let paths = if self.paths.is_empty() {
            let cwd = Path::cwd().try_canonicalize();
            self.println(format!("no paths provided, assuming {}", cwd.abbreviate()), Verbosity::Debug);
            cwd.list().unwrap_or_default()
        } else {
            self.paths.clone()
        };
        let all_paths_are_dirs = paths.iter().all(|path| path.try_canonicalize().is_dir());
        if !self.recursive && all_paths_are_dirs {
            self.eprintln("all target paths are directories but -r/--recursive was not provided", Verbosity::Hint);
        }
        paths
            .into_iter()
            .filter(|path| {
                if !path.exists() {
                    self.eprintln(format!("path does not exist: {path}"), Verbosity::Warning);
                }
                path.exists()
            })
            .collect()
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
            self.eprintln(format!("trying to read {path}"), Verbosity::Debug);
            Ok(path
                .read()?
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| line.len() > 0)
                .collect::<Vec<String>>())
        } else {
            Ok(Vec::new())
        }
    }
    pub fn should_ignore(&self, lines: &Vec<String>, path: &Path) -> Result<bool> {
        if lines.is_empty() {
            Ok(false)
        } else {
            Ok(lines
                .iter()
                .map(|line| line.trim().to_string())
                .filter(|line| line.len() > 0)
                .any(|line| {
                    let line = line.to_string();
                    line == path.name()
                        || line == path.relative_to_cwd().to_string()
                        || line == path.to_string()
                }))
        }
    }
    pub fn unique_new_path(&self, path: &Path) -> Result<Path> {
        let path = path.try_canonicalize();
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
            let new_filename =
                Path::join_extension(format!("{new_name}.{count}"), new_extension.clone());
            new_path = path.with_filename(&new_filename);
            count += 1;
        }
        Ok(new_path)
    }
    pub fn slugify_file_path(&self, path: &Path) -> Result<Path> {
        let path = path.canonicalize()?;
        let new_path = self.unique_new_path(&path)?;
        if path.to_string() != new_path.to_string() {
            if self.dry_run {
                self.println(format!("would rename {path} to {new_path}"), Verbosity::Info);
                return Ok(new_path);
            }
            if path.is_dir() && new_path.is_dir() && self.force {
                if self.debug {
                    dbg!(path.is_dir(), new_path.is_dir(), self.force);
                }
                return Ok(new_path.try_canonicalize());
            } else if path.exists() && new_path.exists() && !self.force {
                return Err(Error::IOError(format!(
                    "{new_path} already exists, use --force to overwrite"
                )));
            }
            let new_path = match path.rename(&new_path, true) {
                Ok(new_path) => new_path,
                Err(error) => return Err(Error::IOError(format!("{}", error))),
            };
            self.println(format!("{path} -> {new_path}"), Verbosity::Info);
            Ok(new_path.try_canonicalize())
        } else {
            if self.debug {
                self.println(format!("'{path}' == '{new_path}'"), Verbosity::Debug);
            } else {
                self.println(format!("unchanged: '{path}'"), Verbosity::Hint);
            }
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

        if cli.debug {
            dbg!(&ignores);
        }
        let (target_paths, total_filtered) = if !cli.force && ignores.len() > 0 {
            let filtered_paths = paths
                .clone()
                .into_iter()
                .filter(|old_path| cli.should_ignore(&ignores, &old_path).unwrap_or_default())
                .collect::<Vec<Path>>();
            let count = filtered_paths.len();
            (filtered_paths.clone(), Some(count))
        } else {
            (paths.clone(), None)
        };
        if cli.debug {
            dbg!(&target_paths, &total_filtered);
        }

        if total_filtered.is_some() && total_filtered.clone().unwrap() == 0 {
            if total_paths > 0 {
                cli.println(format!(
                    "total paths is {total_paths} but all have been ignored: "
                ), Verbosity::Warning);
                for path in paths.iter() {
                    let path = path.relative_to_cwd();
                    cli.println(format!("    {path}"), Verbosity::Warning);
                }
            } else {
                cli.println(format!("no paths to slugify"), Verbosity::Warning);
            }
            return Ok(());
        }
        for old_path in target_paths {
            cli.slugify_path(&old_path)?;
        }
        Ok(())
    }
}
