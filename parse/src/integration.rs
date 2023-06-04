#[cfg(test)]
pub mod tests {
    use std::io::ErrorKind;

    use crate::{error::ParseError, parse_file, Rule};

    #[test]
    fn parse_file_correctly_handles_io_error() {
        let path = "this/path/should/not/exist/ever/lol.pc";

        let result = parse_file(path);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::IOError(_)));
        let ParseError::IOError(io_error) = err else { unreachable!() };
        assert_eq!(io_error.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn parse_file_returns_no_rules_for_empty_file() {
        let path = "fixtures/empty.pc";

        let result = parse_file(path);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn parse_file_reports_error_for_missing_rule() {
        let path = "fixtures/missing-rule.pc";

        let result = parse_file(path);
        assert!(result.is_err());
        let result = result.unwrap_err();
        assert!(matches!(result, ParseError::PestError(_)));
        let ParseError::PestError(inner) = result else { unreachable!(); };
        assert!(matches!(
            inner.variant,
            pest::error::ErrorVariant::ParsingError { .. }
        ));
        let pest::error::ErrorVariant::ParsingError {
            positives,
            negatives,
        } = inner.variant else { unreachable!() };

        assert_eq!(positives.len(), 1);
        assert!(negatives.is_empty());

        assert_eq!(positives.first(), Some(&Rule::rule));
    }
}
