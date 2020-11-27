use crate::{
    Dictionary,
    Error::{DictionaryNotFound, SolitaireNotFound},
    Idiom, Result,
};
use pathfinding::directed::bfs::bfs;
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
    fn get_key(&self, input: &str) -> String {
        let last = input.chars().rev().next().unwrap();
        let key = match self.mode {
            SolitaireMode::Character => last.to_string(),
            SolitaireMode::Tone => last.to_pinyin().unwrap().with_tone().to_string(),
            SolitaireMode::Sound => last.to_pinyin().unwrap().plain().to_string(),
        };
        return key;
    }
    fn successors(&self, state: &Idiom) -> Vec<Idiom> {
        let key = self.get_key(&state.idiom);
        match self.state.get(&key) {
            None => vec![],
            Some(s) => s.to_owned(),
        }
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

    pub fn solve_random(&mut self, input: &str) -> Result<Idiom> {
        if let Some(s) = self.state.get_mut(&self.get_key(input)) {
            if s.is_empty() {
                return Err(SolitaireNotFound);
            }
            let i = (0..s.len()).choose(&mut self.rng)?;
            return Ok(s.swap_remove(i));
        }
        Err(SolitaireNotFound)
    }

    pub fn solve_target(&mut self, input: &str, target: &str) -> Result<Vec<Idiom>> {
        let first = Idiom::from(input);
        let target = target.chars().next()?;
        let target = match self.mode {
            SolitaireMode::Character => target.to_string(),
            SolitaireMode::Tone => target.to_pinyin()?.with_tone().to_string(),
            SolitaireMode::Sound => target.to_pinyin()?.plain().to_string(),
        };
        let result = bfs(&first, |p| self.successors(p), |p| self.get_key(&p.idiom) == target)?;
        Ok(result)
    }

    pub fn solve_greedy(&mut self, input: &str) -> Result<Idiom> {
        let s = match self.state.get(&self.get_key(input)) {
            Some(s) => s,
            None => return Err(SolitaireNotFound),
        };
        if s.is_empty() {
            return Err(SolitaireNotFound);
        }
        let mut max = 0;
        for i in 0..s.len() {
            let this = unsafe { s.get_unchecked(i) };
            let len = match self.state.get(&self.get_key(&this.idiom)) {
                Some(v) => v.len(),
                None => 0,
            };
            if len > max {
                max = i
            }
        }
        let s = self.state.get_mut(&self.get_key(input))?;
        return Ok(s.swap_remove(max));
    }
    pub fn solve_chain(&mut self, head: &str, length: usize) -> Vec<Idiom> {
        let mut out = vec![];
        let mut this = String::from(head);
        for _ in 0..length {
            match self.solve_greedy(&this) {
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
