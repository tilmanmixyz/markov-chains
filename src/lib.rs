use std::ops::Range;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Letter {
    Consonant,
    Vowel,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LetterPair {
    CC,
    CV,
    VV,
    VC,
}

#[derive(Debug, Copy, Clone)]
pub struct LetterPairs {
    pub cc: usize,
    pub cv: usize,
    pub vc: usize,
    pub vv: usize
}

pub struct Letters(pub Vec<Letter>);

impl Letters {
    pub fn new() -> Letters {
        Letters(Vec::new())
    }

    pub fn new_from_str(input: &str) -> MCLResult<Letters> {
        let mut letters = Letters::new();
        letters.update(input)?;
        Ok(letters)
    }

    fn validate(input: &str) -> MCLResult<()> {
        if !input.is_ascii() {
            return Err(MarkovChainLettersError::NotAsciiError);
        }
        for c in input.chars() {
            if !c.is_ascii_alphabetic() {
                if c.is_ascii_digit() {
                    return Err(MarkovChainLettersError::CharacterIsNumberError(c))
                } else {
                    return Err(MarkovChainLettersError::NotAsciiAlphabeticError(c))
                }
            }
        }

        Ok(())
    }

    // TODO: maybe just dilter for alphebetic
    pub fn normalize(input: &str) -> String {
        input.chars().filter(|c| !c.is_ascii_whitespace()).filter(|c| !c.is_ascii_punctuation()).collect()
    }

    pub fn update(&mut self, input: &str) -> MCLResult<()> {
        let input = Letters::normalize(input);
        Letters::validate(&input)?;
        let letters: Vec<Letter> = input.chars().map(|c| match c {
            // Safety: See validate check above
            'a' | 'e' | 'i' | 'o' | 'u' => Letter::Vowel,
            _ => Letter::Consonant
        }).collect();
        self.0.extend_from_slice(&letters);
        Ok(())
    }

    /// (Consonants, Vowels)
    pub fn finalize(&self) -> (usize, usize) {
        let consonents = (&self.0)
            .into_iter()
            .map(|l| match l {
                &Letter::Consonant => 1,
                &Letter::Vowel => 0,
            })
            .sum();
        let vowels = &self.0.len() - consonents;
        (consonents, vowels)
    }

    pub fn analyze_pairs(&self) -> LetterPairs {
        let window = self.0.windows(2);
        let lp = window.map(|w| unsafe {
            match (w.get_unchecked(0), w.get_unchecked(1)) {
                (&Letter::Consonant, &Letter::Consonant) => LetterPair::CC,
                (&Letter::Consonant, &Letter::Vowel) => LetterPair::CV,
                (&Letter::Vowel, &Letter::Consonant) => LetterPair::VC,
                (&Letter::Vowel, &Letter::Vowel) => LetterPair::VV,
            }
        }).collect::<Vec<LetterPair>>();
        let cc: usize = (&lp).into_iter().filter(|lp| *lp == &LetterPair::CC).count();
        let cv: usize = (&lp).into_iter().filter(|lp| *lp == &LetterPair::CV).count();
        let vc: usize = (&lp).into_iter().filter(|lp| *lp == &LetterPair::VC).count();
        let vv: usize = (&lp).into_iter().filter(|lp| *lp == &LetterPair::VV).count();

        LetterPairs {
            cc,cv,vc,vv
        }
    }
}

impl From<Vec<Letter>> for Letters {
    fn from(value: Vec<Letter>) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for Letters {
    type Error = MarkovChainLettersError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Letters::new_from_str(value)
    }
}

enum NextLetterChance {
    C {
        v: f64,
        c: f64,
    },
    V {
        v: f64,
        c: f64
    }
}

pub struct PredictionMachine {
    chances: NextLetterChance,
}

impl PredictionMachine {
    pub fn from_stats(consonants_count: usize, vowels_count: usize, pairs: LetterPairs) -> PredictionMachine {
        
    }
}

pub type MCLResult<T> = Result<T, MarkovChainLettersError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MarkovChainLettersError {
    NotAsciiError,
    CharacterIsNumberError(char),
    NotAsciiAlphabeticError(char),
}

impl core::fmt::Display for MarkovChainLettersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MarkovChainLettersError as M;
        match self {
            M::NotAsciiError => write!(f, "error: input strings contains non-ascii charcters"),
            M::CharacterIsNumberError(c) => write!(f, "error: character {} is a number", c),
            M::NotAsciiAlphabeticError(c) => write!(f, "error: character {} is not a alphabetic character", c),
        }
    }
}

impl core::error::Error for MarkovChainLettersError {}