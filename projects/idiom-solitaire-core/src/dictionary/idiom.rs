use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Idiom {
    #[serde(rename(deserialize = "Idiom"))]
    pub idiom: String,
    #[serde(rename(deserialize = "Pinyin"))]
    pub pinyin: String,
    #[serde(rename(deserialize = "Explanation"))]
    pub explanation: String,
}

impl Idiom {
    pub fn first_char(&self) -> char {
        self.idiom.chars().next().unwrap()
    }
    pub fn last_char(&self) -> char {
        self.idiom.chars().rev().next().unwrap()
    }
}
