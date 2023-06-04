use std::num::ParseFloatError;

use thiserror::Error;

use crate::Rule;

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
    #[error("Unexpected rule({origin}) : {rule}")]
    UnexpectedRule { rule: String, origin: &'static str },
}

pub fn missing(slug: &'static str) -> ParseError {
    ParseError::MissingItem { slug }
}
pub fn unexpected<T>(origin: &'static str, rule: Rule) -> ParseResult<T> {
    Err(ParseError::UnexpectedRule {
        origin,
        rule: format!("{:?}", rule),
    })
}

pub type ParseResult<T> = Result<T, ParseError>;
