use idiom_solitaire::{Dictionary, Idiom, Result};

fn example() -> Result<()> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_path("../database/database-base.csv")?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Idiom = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[test]
fn main() {
    let dict = Dictionary::load_csv("../database/database-base.csv").unwrap();
    println!("{:#?}", dict);
}
