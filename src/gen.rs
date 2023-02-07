use std::ops::Range;

use rand::{seq::SliceRandom, Rng};

use crate::{types::Error, validate_test, Phonet};

impl Phonet {
    /// Generate random words that fit the rules
    pub fn generate(&self, count: usize, length: Range<usize>) -> Result<Vec<String>, Error> {
        //TODO Handle
        let letters = match self.classes.get("_") {
            Some(x) => x,
            None => return Err(Error::MissingAnyClass),
        };

        let mut words = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let word = loop {
                let word = random_word(letters, rng.gen_range(length.clone()));

                if validate_test(&word, &self.rules).is_valid() {
                    break word;
                }
            };

            words.push(word);
        }

        Ok(words)
    }
}

/// Generate random word with given characters and given length
/// TODO replace `letters` type with impl IntoIterator with char type or something
fn random_word(letters: &str, length: usize) -> String {
    let chars: Vec<char> = letters.chars().collect();
    let mut word = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        word.push(*chars.choose(&mut rng).unwrap());
    }

    word
}
