// TODO: Remove this once MacroDocumentParser has more usages.
#![allow(dead_code)]
#![allow(unused_macros)]

#[derive(pest_derive::Parser)]
#[grammar = "scroll.pest"]
pub struct DocumentParser;

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser as _;

    macro_rules! assert_all_rule {
        ($rule:expr, $in:expr) => {
            for input in $in {
                let len = match DocumentParser::parse($rule, input) {
                    Ok(rule) => rule.last().unwrap().as_span().end(),
                    Err(err) => panic!(format!("{}", err)),
                };

                assert_eq!(len, input.len(), "Failed to assert that rules match for: {}", input);
            }
        };
    }

    macro_rules! assert_all_not_rule {
        ($rule:expr, $in:expr) => {
            assert!($in.iter().all(|input| {
                DocumentParser::parse($rule, input).is_err()
                    || DocumentParser::parse($rule, input).unwrap().last().unwrap().as_span().end() != input.len()
            }));
        };
    }

    #[test]
    fn parse_accepted_complex_text_macros() {
        let inputs = &[
            include_str!("../test/data/nonsense.txt"),
            include_str!("../test/data/more_nonsense.txt"),
            include_str!("../test/data/long_sword_basic_attack.txt"),
            include_str!("../test/data/long_sword_multiple_attack.txt"),
            include_str!("../test/data/eblast.txt"),
        ];

        assert_all_rule!(Rule::document, inputs);
    }
}
