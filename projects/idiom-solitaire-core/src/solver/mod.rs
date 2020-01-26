use crate::{
    Dictionary,
    Error::{DictionaryNotFound, SolitaireNotFound},
    Idiom, Result,
};
use pinyin::ToPinyin;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
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
    pub rng: SmallRng,
}

impl Default for SolitaireSolver {
    fn default() -> Self {
        Self { dict: Default::default(), state: Default::default(), mode: SolitaireMode::Tone, rng: SmallRng::from_entropy() }
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

    pub fn solve(&mut self, input: &str) -> Result<Idiom> {
        let head = input.chars().rev().next()?;
        let key = match self.mode {
            SolitaireMode::Character => head.to_string(),
            SolitaireMode::Pinyin => head.to_pinyin()?.with_tone().to_string(),
            SolitaireMode::Tone => head.to_pinyin()?.plain().to_string(),
        };
        if let Some(s) = self.state.get_mut(&key) {
            if s.is_empty() {
                return Err(SolitaireNotFound);
            }
            let i = (0..s.len()).choose(&mut self.rng)?;
            return Ok(s.swap_remove(i));
        }
        Err(SolitaireNotFound)
    }

    pub fn solve_chain(&mut self, head: &str, length: usize) -> (Vec<Idiom>, bool) {
        let mut out = vec![];
        let mut success = true;
        let mut this = String::from(head);
        for _ in 0..length {
            match self.solve(&this) {
                Ok(o) => {
                    this = o.idiom.clone();
                    out.push(o)
                }
                Err(_) => {
                    success = false;
                    break;
                }
            }
        }
        return (out, success);
    }
}
