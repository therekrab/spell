use rspell::{spellcheck, Correction};

pub struct Token {
    literal: String,
    line: usize,
}

#[derive(Debug, PartialEq)]
enum Capitalization {
    LowerAll,
    UpperAll,
    First,
    Other,
}

impl Capitalization {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            return Self::Other;
        }
        let mut first = true;
        let mut lowercase = true;
        let mut uppercase = true;
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() {
                lowercase = false;
                if i > 0 {
                    first = false;
                }
            } else {
                uppercase = false;
                if i == 0 {
                    first = false;
                }
            }
        }
        if first {
            return Self::First;
        }
        if lowercase {
            return Self::LowerAll;
        }
        if uppercase {
            return Self::UpperAll;
        }
        Self::Other
    }

    fn apply(&self, s: &str) -> String {
        match self {
            Self::Other => s.to_string(),
            Self::LowerAll => s.to_lowercase(),
            Self::UpperAll => s.to_uppercase(),
            Self::First => format!("{}{}", s[..1].to_uppercase(), s[1..].to_lowercase()),
        }
    }
}

impl Token {
    fn new(literal: &str, line: usize) -> Self {
        Token {
            literal: literal.to_string(),
            line,
        }
    }

    pub fn format(&self, dictionary: &[String]) -> Option<String> {
        match spellcheck(&self.literal.to_lowercase(), dictionary) {
            Correction::Correct => None,
            Correction::Recommendation(r) => Some(format!(
                "line {}: {:?} => {}",
                self.line + 1,
                self.literal,
                Capitalization::from(&self.literal).apply(r)
            )),
            Correction::Unknown => Some(format!(
                "line {}: {} (no match found)",
                self.line + 1,
                self.literal
            )),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (line, text) in input.lines().enumerate() {
        for word in text.split_whitespace() {
            tokens.append(&mut tokenize_word(word, line));
        }
    }
    tokens
}

fn tokenize_word(word: &str, line: usize) -> Vec<Token> {
    if word.is_empty() {
        return Vec::new();
    }
    if !word.is_ascii() {
        panic!("Found non-ASCII word: {word}. This app doesn't currently support non-ascii characters.");
    }
    let mut result = Vec::new();
    // Pull characters until we reach an invalid character
    let mut start = 0;
    for (i, c) in word.chars().enumerate() {
        if !(c.is_ascii_alphabetic() || c == '\'') {
            // If this is the starting index as well as the ending, we actually just hit another
            // bad character.
            if i != start {
                result.push(Token::new(&word[start..i], line));
            }
            start = i + 1;
        }
    }
    if start < word.len() {
        result.push(Token::new(&word[start..], line));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let results = tokenize("hello, MY name: is Bob.");
        assert_eq!(results[0].literal, "hello");
        assert_eq!(results[1].literal, "MY");
        assert_eq!(results[2].literal, "name");
        assert_eq!(results[3].literal, "is");
        assert_eq!(results[4].literal, "Bob");
    }

    #[test]
    fn test_cap() {
        let first = Capitalization::from("First");
        let upper = Capitalization::from("ALL");
        let lower = Capitalization::from("lower");
        let other = Capitalization::from("OtHeR");

        assert_eq!(first, Capitalization::First);
        assert_eq!(upper, Capitalization::UpperAll);
        assert_eq!(lower, Capitalization::LowerAll);
        assert_eq!(other, Capitalization::Other);

        assert_eq!(first.apply("HELLO"), "Hello");
        assert_eq!(upper.apply("hello"), "HELLO");
        assert_eq!(lower.apply("HElLO"), "hello");
        assert_eq!(other.apply("HeLLO"), "HeLLO");
    }
}
