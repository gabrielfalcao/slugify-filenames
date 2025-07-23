use slugify::{Result, SlugifyString};


fn main() -> Result<()> {
    Ok(SlugifyString::execute(
        std::env::args().map(|c| c.to_string()).collect(),
    )?)
}
