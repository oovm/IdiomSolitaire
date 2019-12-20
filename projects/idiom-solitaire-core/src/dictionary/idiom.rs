use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Idiom {
    #[serde(rename(deserialize = "Idiom"))]
    pub idiom: String,
    #[serde(rename(deserialize = "Pinyin"))]
    pub pinyin: String,
    #[serde(rename(deserialize = "Letter"))]
    pub letter: String,
    #[serde(rename(deserialize = "Explanation"))]
    pub explanation: String,
    #[serde(rename(deserialize = "Synonym"))]
    pub synonym: String,
}

impl Idiom {
    pub fn first_char(&self) -> char {
        self.idiom.chars().next().unwrap()
    }
    pub fn last_char(&self) -> char {
        self.idiom.chars().rev().next().unwrap()
    }
    pub fn first_pinyin(&self) -> String {
        self.pinyin.split(' ').next().unwrap().to_owned()
    }
    pub fn last_pinyin(&self) -> String {
        self.pinyin.split(' ').rev().next().unwrap().to_owned()
    }
    pub fn first_tone(&self) -> String {
        self.letter.split(' ').next().unwrap().to_owned()
    }
    pub fn last_tone(&self) -> String {
        self.letter.split(' ').rev().next().unwrap().to_owned()
    }
}
