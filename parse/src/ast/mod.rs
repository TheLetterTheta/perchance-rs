use pest::iterators::Pair;

use crate::{error::ParseResult, Rule};

pub mod item;
pub mod rule;

pub trait Parse: Sized {
    fn parse(line: Pair<Rule>) -> ParseResult<Self>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub content: String,
    pub start: usize,
    pub end: usize,
}

pub fn get_span(line: &Pair<Rule>) -> Span {
    let s = line.as_span();
    Span {
        content: s.as_str().to_owned(),
        end: s.end(),
        start: s.start(),
    }
}
