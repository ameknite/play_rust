use color_eyre::eyre;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    println!("Play rust!");

    Ok(())
}
