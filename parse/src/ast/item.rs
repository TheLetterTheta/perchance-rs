use log::trace;
use pest::iterators::Pair;

use crate::{ast::get_span, error::ParseResult, next, validate_rule, Rule};

use super::{rule::PerchanceRule, Parse, Span};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    name: String,
    span: Span,
    rules: Vec<PerchanceRule>,
}
impl Parse for Item {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-item");
        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), section);
        trace!("[EndOf:1] validate-rule");

        let span = get_span(&line);
        let mut rules = line.into_inner();

        trace!("[Start:2] get-rules");
        let name = next!(rules, "item-name");
        let rules = next!(rules, "item-rule(s)");
        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] read-name");
        let name = name.as_str().to_string();
        trace!("[EndOf:3] read-name({name})");

        trace!("[Start:4] parse-rules");
        let rules = rules
            .into_inner()
            .map(PerchanceRule::parse)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[EndOf:4] parse-rules");

        trace!("[EndOf] parse-item");
        Ok(Self { name, span, rules })
    }
}
