use pest::iterators::Pair;

use crate::{
    error::{wot, ParseResult},
    next, validate_rule, Rule,
};

use super::{rule::PerchanceRule, Parse};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    name: String,
    rules: Vec<PerchanceRule>,
}
impl Parse for Item {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        validate_rule!(line.as_rule(), section);

        let mut rules = line.into_inner();

        let name = next!(rules, "item-name");
        let rules = next!(rules, "item-rule(s)");
        let rules = rules
            .into_inner()
            .map(PerchanceRule::parse)
            .collect::<ParseResult<Vec<_>>>()?;

        Ok(Self {
            name: name.as_str().to_string(),
            rules,
        })
    }
}
