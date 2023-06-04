use log::trace;
use pest::iterators::Pair;
use serde::Serialize;

use crate::{ast::get_span, error::ParseResult, next, validate_rule, Rule};

use super::{rule::PerchanceRule, Parse, Span};

#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    #[serde(skip)]
    pub span: Span,
    pub rules: Vec<PerchanceRule>,
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

#[cfg(test)]
pub mod tests {
    use crate::parse_file;

    macro_rules! assert_rule_raw_with_value {
        ($rule:expr, $value:literal) => {{
            let __rule = $rule;
            assert!(__rule.is_some());
            let Some(__rule) = __rule else { unreachable!() };
            let __rule = __rule.as_ref();
            assert!(matches!(
                __rule,
                $crate::ast::rule::PerchanceRuleInner::Raw(_)
            ));
            let $crate::ast::rule::PerchanceRuleInner::Raw(__rule) = __rule else { unreachable!() };
            assert_eq!(__rule, $value);
        }};
    }

    macro_rules! expect_item_like {
        ($item:expr, $name:literal, [$value:literal $(, $rest:literal)*]) => {{
            let __item = $item;
            assert!(__item.is_some());
            let Some(__item) = __item else { unreachable!() };
            let __expected_values = vec![$value,$($rest,)*];

            assert_eq!(__item.name, $name);
            assert_eq!(__item.rules.len(), __expected_values.len());

            let mut __rules = __item.rules.into_iter();

            assert_rule_raw_with_value!(__rules.next(), $value);
            $(
                assert_rule_raw_with_value!(__rules.next(), $rest);
            )*

        }};
    }

    #[test]
    fn parse_item_single_item_parses_correctly() {
        let result = parse_file("fixtures/items/one-item.pc");
        assert!(result.is_ok());
        let Ok(items) = result else { unreachable!() };
        assert_eq!(items.len(), 1);
        expect_item_like!(items.first().cloned(), "pack", ["backpack", "bag"]);
    }

    #[test]
    fn parse_item_two_items_parses_correctly() {
        let result = parse_file("fixtures/items/two-items.pc");
        assert!(result.is_ok());
        let Ok(items) = result else { unreachable!() };
        assert_eq!(items.len(), 2);
        let mut items = items.into_iter();

        expect_item_like!(items.next(), "pack", ["backpack", "bag"]);
        expect_item_like!(items.next(), "fruit", ["apple", "orange"]);
    }

    #[test]
    fn parse_item_inline_rule_parses_correctly() {
        let result = parse_file("fixtures/items/inline-rule.pc");
        assert!(result.is_ok());
        let Ok(items) = result else { unreachable!() };
        assert_eq!(items.len(), 1);
        let mut items = items.into_iter();

        expect_item_like!(items.next(), "pack", ["bag"]);
    }
}
