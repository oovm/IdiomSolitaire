use crate::{Dictionary, Error::DictionaryNotFound, Idiom, Result};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum SolitaireMode {
    Character = 0,
    Pinyin = 1,
    Tone = 2,
}

#[derive(Clone, Debug)]
pub struct SolitaireSolver {
    pub dict: Dictionary,
    pub state: HashMap<String, Vec<Idiom>>,
    pub mode: SolitaireMode,
}

impl Default for SolitaireSolver {
    fn default() -> Self {
        Self { dict: Default::default(), state: Default::default(), mode: SolitaireMode::Tone }
    }
}

impl SolitaireSolver {
    pub fn length(&self) -> usize {
        self.dict.0.len()
    }
}

impl SolitaireSolver {
    pub fn load(&mut self, bytes: &[u8]) -> Result<()> {
        self.dict = Dictionary::load_bytes(bytes)?;
        Ok(self.refresh()?)
    }
    pub fn refresh(&mut self) -> Result<()> {
        if self.length() == 0 {
            return Err(DictionaryNotFound);
        }
        match self.mode {
            SolitaireMode::Character => self.state = self.dict.char_map(),
            SolitaireMode::Pinyin => self.state = self.dict.pinyin_map(),
            SolitaireMode::Tone => self.state = self.dict.tone_map(),
        }
        Ok(())
    }

    pub fn solve(&mut self) {

    }
}
