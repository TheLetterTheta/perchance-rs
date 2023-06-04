use log::trace;
use pest::iterators::Pair;
use serde::Serialize;

use crate::{
    ast::get_span,
    error::{wot, ParseError, ParseResult},
    next, validate_rule, Rule,
};

use super::{Parse, Span};

#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub enum PerchanceRuleInner {
    Odds {
        modifier: f32,
    },
    Options(Vec<Box<PerchanceRule>>),
    Reference {
        chain: Vec<Box<PerchanceRule>>,
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

#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub struct PerchanceRule {
    #[serde(skip)]
    span: Span,
    #[serde(flatten)]
    inner: PerchanceRuleInner,
}
impl PerchanceRule {
    fn map_boxed(slf: ParseResult<Self>) -> ParseResult<Box<Self>> {
        Ok(Box::new(slf?))
    }
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
    fn parse_boxed(line: Pair<Rule>) -> ParseResult<Box<Self>> {
        Ok(Box::new(Self::parse(line)?))
    }
    fn parse_raw(line: Pair<Rule>) -> ParseResult<Self> {
        validate_rule!(line.as_rule(), sector_raw);

        let span = get_span(&line);
        let inner = line.as_str().to_owned();
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Raw(inner),
        })
    }
    fn parse_name(line: Pair<Rule>) -> ParseResult<Self> {
        validate_rule!(line.as_rule(), name);
        let span = get_span(&line);

        let name = line.as_str().to_owned();

        Ok(Self {
            span,
            inner: PerchanceRuleInner::Name(name),
        })
    }
    fn parse_odds(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-odds");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), sector_odds);
        trace!("[EndOf:1] validate-rule");

        let span = get_span(&line);

        trace!("[Start:2] get-modifier");
        let mut rules = line.into_inner();
        let number = next!(rules, "rule-odds-number");
        validate_rule!(number.as_rule(), number);
        trace!("[EndOf:2] get-modifier");

        trace!("[Start:3] parse-modifier");
        let modifier_raw = number.as_str();
        let modifier = str::parse(modifier_raw)?;
        trace!("[EndOf:3] parse-modifier");

        trace!("[EndOf] parse-odds");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Odds { modifier },
        })
    }

    fn parse_reference(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-reference");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), sector_reference);
        trace!("[EndOf:1] validate-rule");

        let span = get_span(&line);

        trace!("[Start:2] get-references");
        let rules = line.into_inner();
        let chain = rules
            .map(Self::parse_boxed)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[Start:2] get-references");

        trace!("[EndOf] parse-reference");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Reference { chain },
        })
    }

    fn parse_compound(line: Pair<Rule>) -> ParseResult<PerchanceRule> {
        // TODO: validate_rule!
        // validate_rule!(line.as_rule(), rule);

        let span = get_span(&line);
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

    fn parse_shorthand(line: Pair<Rule>) -> ParseResult<PerchanceRule> {
        trace!("[Start] parse-shorthand");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), sector_shorthand);
        trace!("[EndOf:1] validate-rule");

        let span = get_span(&line);

        trace!("[Start:2] get-options");
        let rules = line.into_inner();
        let options = rules
            .map(Self::parse_boxed)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[Start:2] get-options");

        trace!("[EndOf] parse-shorthand");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Options(options),
        })
    }

    fn parse_import(line: Pair<Rule>) -> Result<PerchanceRule, ParseError> {
        validate_rule!(line.as_rule(), import);
        let span = get_span(&line);

        let mut rules = line.into_inner();
        let generator = next!(rules, "rule-import-generator");
        validate_rule!(generator.as_rule(), generator_name);
        let generator = generator.as_str().to_owned();

        Ok(Self {
            span,
            inner: PerchanceRuleInner::Import { generator },
        })
    }
}
impl Parse for PerchanceRule {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-rule");

        trace!("[Start:1] validate-rule");
        validate_rule!(
            line.as_rule(),
            rule,
            name,
            import,
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
            Rule::name => Self::parse_name(line),
            Rule::sector_odds => Self::parse_odds(line),
            Rule::sector_reference => Self::parse_reference(line),
            Rule::sector_shorthand => Self::parse_shorthand(line),
            Rule::import => Self::parse_import(line),
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
