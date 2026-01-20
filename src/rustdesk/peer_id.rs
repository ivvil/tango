use std::{fmt::Display, hash::Hash, str::FromStr};

use thiserror::Error;

const MIN_LENGTH: usize = 6;
const MAX_LENGTH: usize = 16;

const VALID_CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '-', '_',
];

#[derive(Debug, Error)]
pub enum PeerIdError {
    #[error("Peer ID is empty")]
    Empty,

    #[error("Peer ID is too short")]
    TooShort,

    #[error("Peer ID is too long")]
    TooLong,

    #[error("Peer ID contains invalid characters. Valid characters: a-z A-Z 0-9 _ -")]
    InvalidCharacters,
}

#[derive(Debug, Clone)]
pub struct PeerId(String);

impl PeerId {
    pub fn new(id: &str) -> Result<Self, PeerIdError> {
        Self::validate(id)?;
        Ok(Self(id.to_string()))
    }

    pub fn validate(id: &str) -> Result<(), PeerIdError> {
        if id.is_empty() {
            return Err(PeerIdError::Empty);
        }

        if id.len() < MIN_LENGTH {
            return Err(PeerIdError::TooShort);
        }

        if id.len() > MAX_LENGTH {
            return Err(PeerIdError::TooLong);
        }

        if !id.chars().all(|c| VALID_CHARS.contains(&c)) {
            return Err(PeerIdError::InvalidCharacters);
        }

        Ok(())
    }
}

impl Default for PeerId {
    fn default() -> Self {
        Self("".into())
    }
}

impl TryFrom<String> for PeerId {
    type Error = PeerIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<&str> for PeerId {
    type Error = PeerIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for PeerId {
    type Err = PeerIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl PartialEq for PeerId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PeerId {}

impl Hash for PeerId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Display for PeerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
