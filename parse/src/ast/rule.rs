use log::trace;
use pest::iterators::Pair;

use crate::{
    ast::span,
    error::{wot, ParseError, ParseResult},
    macros::validate_rule,
    next, validate_rule, Rule,
};

use super::{Parse, Span};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PerchanceRuleInner {
    Odds {
        modifier: f32,
    },
    Options(Vec<Box<PerchanceRule>>),
    Reference {
        parent: Box<PerchanceRule>,
        child: Box<PerchanceRule>,
    },
    Import {
        generator: String,
    },
    Store {
        name: String,
        value: Box<PerchanceRule>,
    },
    Name(String),
    Raw(String),
    Compound(Vec<Box<PerchanceRule>>),
    #[default]
    Nop,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PerchanceRule {
    span: Span,
    inner: PerchanceRuleInner,
}
impl PerchanceRule {
    fn parse_boxed(line: Pair<Rule>) -> ParseResult<Box<Self>> {
        Ok(Box::new(Self::parse(line)?))
    }
    fn parse_raw(line: Pair<Rule>) -> ParseResult<Self> {
        validate_rule!(line.as_rule(), sector_raw);

        let span = span(&line);
        let inner = line.as_str().to_owned();
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Raw(inner),
        })
    }
    fn parse_odds(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-odds");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), sector_odds);
        trace!("[EndOf:1] validate-rule");

        let span = span(&line);

        trace!("[Start:2] get-modifier");
        let mut rules = line.into_inner();
        let number = next!(rules, "rule-odds-number");
        validate_rule!(number.as_rule(), number);
        trace!("[Start:2] get-modifier");

        trace!("[Start:2] parse-modifier");
        let modifier_raw = number.as_str();
        let modifier = str::parse(modifier_raw)?;
        trace!("[Start:2] parse-modifier");

        trace!("[EndOf] parse-odds");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Odds { modifier },
        })
    }

    fn parse_compound(line: Pair<Rule>) -> Result<PerchanceRule, ParseError> {
        // TODO: validate_rule!
        // validate_rule!(line.as_rule(), rule);

        let span = span(&line);
        match line.as_rule() {
            Rule::rule => {
                let rules = line.into_inner().collect::<Vec<_>>();
                if rules.len() == 1 {
                    return Self::parse(rules.into_iter().next().unwrap());
                }

                let rules = rules
                    .into_iter()
                    .map(Self::parse_boxed)
                    .collect::<ParseResult<Vec<_>>>()?;

                Ok(Self {
                    span,
                    inner: PerchanceRuleInner::Compound(rules),
                })
            }
            _ => wot("parse-compound"),
        }
    }
}
impl Parse for PerchanceRule {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-rule");

        trace!("[Start:1] validate-rule");
        validate_rule!(
            line.as_rule(),
            rule,
            sector_raw,
            sector_reference,
            reference_name,
            sector_odds,
            sector_shorthand
        );
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] dispatch ({:?})", line.as_rule());
        let rule = match line.as_rule() {
            Rule::rule => Self::parse_compound(line),
            Rule::sector_raw => Self::parse_raw(line),
            Rule::sector_odds => Self::parse_odds(line),
            _ => wot("parse-rule"),
        };
        trace!("[EndOf:2] dispatch");

        trace!("[EndOf] parse-rule");
        rule
    }
}
impl std::ops::Deref for PerchanceRule {
    type Target = PerchanceRuleInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AsRef<PerchanceRuleInner> for PerchanceRule {
    fn as_ref(&self) -> &PerchanceRuleInner {
        &self.inner
    }
}
