mod idiom;

use crate::Result;

use csv::StringRecord;
pub use idiom::Idiom;
use itertools::Itertools;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Dictionary(pub Vec<Idiom>);

impl Default for Dictionary {
    fn default() -> Self {
        Self(vec![])
    }
}

impl Dictionary {
    pub fn load_csv(path: impl AsRef<Path>) -> Result<Self> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_path(path)?;
        let mut out = vec![];
        for result in rdr.deserialize() {
            let record: Idiom = result?;
            out.push(record)
        }
        Ok(Self(out))
    }
}
