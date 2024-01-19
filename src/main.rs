use color_eyre::eyre;
use itertools::Itertools;
use tap::prelude::*;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    println!("hello, world");

    Ok(())
}
