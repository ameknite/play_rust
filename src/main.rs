use color_eyre::eyre;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let x = 10;
    println!("hello, world");

    Ok(())
}
