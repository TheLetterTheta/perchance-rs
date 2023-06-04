#[macro_export]
macro_rules! validate_rule {
    ($actual:expr, $expected:ident) => {
        if !matches!($actual, Rule::$expected) {
            log::error!(
                "invalid-rule: {:?} != {:?}",
                $crate::Rule::$expected,
                $actual
            );
            return $crate::error::ParseResult::Err(
                $crate::error::ParseError::InvalidRuleError {
                    expected: Rule::$expected,
                    actual: $actual,
                },
            );
        }
    };

    ($actual:expr, $expected_first:ident $(, $expected_rest:ident)+) => {{
        let __options = vec![
            $crate::Rule::$expected_first,
            $($crate::Rule::$expected_rest,)+
        ];
        if !__options.iter().any(|rule| *rule == $actual) {
            log::error!(
                "invalid-rule(set): {:?} not in {:?}",
                $actual,
                __options,
            );
            return $crate::error::ParseResult::Err(
                $crate::error::ParseError::InvalidRuleErrorOneOf {
                    expected: __options,
                    actual: $actual,
                },
            );
        }}
    };
}
