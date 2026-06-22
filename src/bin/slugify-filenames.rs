use slugify::{Result, SlugifyFilenames};

fn main() -> Result<()> {
    env_logger::init();

    Ok(SlugifyFilenames::execute(
        std::env::args()
            .map(|c| c.to_string())
            .collect::<Vec<String>>(),
    )?)
}
