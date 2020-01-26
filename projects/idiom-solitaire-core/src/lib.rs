#![feature(try_trait)]

mod dictionary;
mod errors;
mod solver;

pub use dictionary::{Dictionary, Idiom};
pub use errors::{Error, Result};
pub use solver::{SolitaireMode, SolitaireSolver};
