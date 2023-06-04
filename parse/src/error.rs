use std::num::ParseFloatError;

use thiserror::Error;

use crate::Rule;

use super::ast::Span;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid rule. Expected {expected:?} got {actual:?}")]
    InvalidRuleError { expected: Rule, actual: Rule },
    #[error("Invalid rule. Expected {expected:?} got {actual:?}")]
    InvalidRuleErrorOneOf { expected: Vec<Rule>, actual: Rule },
    #[error("Missing item. {slug}")]
    MissingItem { slug: &'static str },
    #[error("Error parsing float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("wot {message}")]
    WotError { message: &'static str },
}

pub fn missing(slug: &'static str) -> ParseError {
    ParseError::MissingItem { slug }
}
pub fn wot<T>(message: &'static str) -> ParseResult<T> {
    Err(ParseError::WotError { message })
}

pub type ParseResult<T> = Result<T, ParseError>;
