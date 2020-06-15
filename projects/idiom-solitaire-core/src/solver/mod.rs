use crate::{
    Dictionary,
    Error::{DictionaryNotFound, SolitaireNotFound},
    Idiom, Result,
};
use pinyin::ToPinyin;
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

#[derive(Clone, Debug)]
pub enum SolitaireMode {
    Character = 0,
    Tone = 1,
    Sound = 2,
}

#[derive(Clone)]
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

impl Debug for SolitaireSolver {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self { dict, state, mode, rng: _ } => f
                .debug_struct("SolitaireSolver")
                .field("dict", &dict.0.len())
                .field("state", &state.len())
                .field("mode", mode)
                .finish(),
        }
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
        self.rng = SmallRng::from_entropy();
        match self.mode {
            SolitaireMode::Character => self.state = self.dict.char_map(),
            SolitaireMode::Sound => self.state = self.dict.sound_map(),
            SolitaireMode::Tone => self.state = self.dict.tone_map(),
        }
        Ok(())
    }

    pub fn solve(&mut self, input: &str) -> Result<Idiom> {
        let head = input.chars().rev().next()?;
        let key = match self.mode {
            SolitaireMode::Character => head.to_string(),
            SolitaireMode::Sound => head.to_pinyin()?.with_tone().to_string(),
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

    pub fn solve_chain(&mut self, head: &str, length: usize) -> Vec<Idiom> {
        let mut out = vec![];
        let mut this = String::from(head);
        for _ in 0..length {
            match self.solve(&this) {
                Ok(o) => {
                    this = o.idiom.clone();
                    out.push(o)
                }
                Err(_) => {
                    break;
                }
            }
        }
        return out;
    }
}
