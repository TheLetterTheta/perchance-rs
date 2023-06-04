use log::trace;
use pest::iterators::Pair;
use serde::Serialize;

use crate::{
    ast::get_span,
    error::{unexpected, ParseResult},
    next, validate_rule, Rule,
};

use super::{Parse, Span};

#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub enum PerchanceRuleInner {
    Odds {
        modifier: f32,
    },
    Options(Vec<PerchanceRule>),
    Reference {
        chain: Vec<PerchanceRule>,
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
    Compound(Vec<PerchanceRule>),
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
    fn parse_boxed(line: Pair<Rule>) -> ParseResult<Box<Self>> {
        Ok(Box::new(Self::parse(line)?))
    }
    fn parse_raw(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-raw");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), sector_raw);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-content");
        let span = get_span(&line);
        let inner = line.as_str().to_owned();
        trace!("[EndOf:2] get-content({inner})");

        trace!("[EndOf] parse-raw");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Raw(inner),
        })
    }
    fn parse_name(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-name");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), name);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-content");
        let span = get_span(&line);
        let name = line.as_str().to_owned();
        trace!("[EndOf:2] get-content({name})");

        trace!("[EndOf] parse-name");

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
        validate_rule!(line.as_rule(), reference_name);
        trace!("[EndOf:1] validate-rule");

        let span = get_span(&line);

        trace!("[Start:2] get-references");
        let rules = line.into_inner();
        let chain = rules.map(Self::parse).collect::<ParseResult<Vec<_>>>()?;
        trace!("[Start:2] get-references");

        trace!("[EndOf] parse-reference");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Reference { chain },
        })
    }

    fn parse_compound(line: Pair<Rule>) -> ParseResult<PerchanceRule> {
        trace!("[Start] parse-compound");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), rule);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-rules");
        let span = get_span(&line);
        let rules = line.into_inner().collect::<Vec<_>>();
        trace!("[EndOf:2] get-rules");

        if rules.len() == 1 {
            trace!("[EndOf] parse-compound(single-rule)");

            // Unwrap is safe because of the condition
            return Self::parse(rules.into_iter().next().unwrap());
        }

        trace!("[Start:3] parse-rules(is-compound)");
        let rules = rules
            .into_iter()
            .map(Self::parse)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[EndOf:3] parse-rules(is-compound)");

        trace!("[EndOf] parse-compound");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Compound(rules),
        })
    }

    fn parse_shorthand(line: Pair<Rule>) -> ParseResult<PerchanceRule> {
        trace!("[Start] parse-shorthand");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), sector_shorthand);
        trace!("[EndOf:1] validate-rule");

        let span = get_span(&line);

        trace!("[Start:2] get-options");
        let rules = line.into_inner();
        let options = rules.map(Self::parse).collect::<ParseResult<Vec<_>>>()?;
        trace!("[Start:2] get-options");

        trace!("[EndOf] parse-shorthand");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Options(options),
        })
    }

    fn parse_import(line: Pair<Rule>) -> ParseResult<PerchanceRule> {
        trace!("[Start] parse-import");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), import);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-rules");
        let span = get_span(&line);
        let mut rules = line.into_inner();
        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] get-generator");
        let generator = next!(rules, "rule-import-generator");
        validate_rule!(generator.as_rule(), generator_name);
        let generator = generator.as_str().to_owned();
        trace!("[EndOf:3] get-generator({generator})");

        trace!("[EndOf] parse-import");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Import { generator },
        })
    }

    fn parse_store(line: Pair<Rule>) -> ParseResult<PerchanceRule> {
        trace!("[Start] parse-store");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), sector_store);
        trace!("[EndOf:1] validate-rule");

        let span = get_span(&line);

        trace!("[Start:2] get-rules");
        let mut rules = line.into_inner();
        let key = next!(rules, "rule-store-name");
        let value = next!(rules, "rule-store-value");
        validate_rule!(key.as_rule(), name);
        validate_rule!(value.as_rule(), store_value);
        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] parse-modifier");
        let name = key.as_str().to_owned();
        let value = Self::parse_boxed(value)?;
        trace!("[EndOf:3] parse-modifier");

        trace!("[EndOf] parse-store");
        Ok(Self {
            span,
            inner: PerchanceRuleInner::Store { name, value },
        })
    }

    fn is_wrapper(rule: Rule) -> bool {
        matches!(rule, Rule::sector_reference | Rule::store_value)
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
            sector_store,
            store_value,
            sector_shorthand
        );
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] dispatch ({:?})", line.as_rule());
        let rule = match line.as_rule() {
            Rule::rule => Self::parse_compound(line),
            Rule::sector_raw => Self::parse_raw(line),
            Rule::name => Self::parse_name(line),
            Rule::sector_odds => Self::parse_odds(line),
            Rule::reference_name => Self::parse_reference(line),
            Rule::sector_shorthand => Self::parse_shorthand(line),
            Rule::sector_store => Self::parse_store(line),
            Rule::import => Self::parse_import(line),
            rule if Self::is_wrapper(rule) => Self::parse(line.into_inner().next().unwrap()),
            rule => unexpected("parse-rule", rule),
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
