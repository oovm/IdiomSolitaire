mod idiom;

use crate::Result;

pub use idiom::Idiom;
use itertools::Itertools;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

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
    pub fn load_bytes(bytes: &[u8]) -> Result<Self> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(bytes);
        let mut out = vec![];
        for result in rdr.deserialize() {
            let record: Idiom = result?;
            out.push(record)
        }
        Ok(Self(out))
    }
}

impl Dictionary {
    pub fn char_map(&self) -> HashMap<String, Vec<Idiom>> {
        self.0.iter().map(|e| (e.first_char().to_string(), e.clone())).into_group_map()
    }
    pub fn sound_map(&self) -> HashMap<String, Vec<Idiom>> {
        self.0.iter().map(|e| (e.first_sound(), e.clone())).into_group_map()
    }
    pub fn tone_map(&self) -> HashMap<String, Vec<Idiom>> {
        self.0.iter().map(|e| (e.first_tone(), e.clone())).into_group_map()
    }
}
