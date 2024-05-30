use color_eyre::eyre::bail;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let dna = "ACTG";
    let result = to_rna(dna);

    let x = "hello".to_string();
    let _ = something(x);

    match result {
        Ok(rna) => println!("RNA: {}", rna),
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}

fn to_rna(dna: &str) -> color_eyre::Result<String> {
    dna.chars().try_fold(String::new(), |mut rna, nucleotide| {
        let complement = complement(nucleotide)?;
        rna.push(complement);
        Ok(rna)
    })
}

fn complement(nucleotide: char) -> color_eyre::Result<char> {
    match nucleotide {
        'A' => Ok('U'),
        'C' => Ok('G'),
        'G' => Ok('C'),
        'T' => Ok('A'),
        _ => bail!("Invalid DNA nucleotide: {nucleotide}"),
    }
}

// fn to_rna2(dna: &str) -> color_eyre::Result<String> {
//     Ok(dna
//         .chars()
//         .map(complement)
//         .collect::<color_eyre::Result<Vec<char>>>()?
//         .into_iter()
//         .collect())
// }

#[must_use]
fn something(hello: String) -> String {
    hello
}
