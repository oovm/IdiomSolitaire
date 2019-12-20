use idiom_solitaire::Dictionary;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    let dict = Dictionary::load_csv("../external/database.csv").unwrap();
    println!("{:#?}", dict.tone_map());
}
