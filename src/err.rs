use std::{fmt::Display, time::SystemTimeError};

pub type CacheResult<T> = Result<T, CacheErr>;

#[derive(Debug, Clone)]
pub enum CacheErr {
    InternalError(String),
    ExternalError(String),
}

impl Display for CacheErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheErr::InternalError(message) => write!(f, "{}", message),
            CacheErr::ExternalError(message) => write!(f, "External Error: {}", message),
        }
    }
}

// impl std::error::Error for CacheErr {}

impl From<SystemTimeError> for CacheErr {
    fn from(err: SystemTimeError) -> Self {
        CacheErr::InternalError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for CacheErr {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        CacheErr::InternalError(err.to_string())
    }
}

impl From<serde_json::Error> for CacheErr {
    fn from(err: serde_json::Error) -> Self {
        CacheErr::InternalError(err.to_string())
    }
}
