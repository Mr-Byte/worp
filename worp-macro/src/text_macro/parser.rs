// TODO: Remove this once TextMacroDocumentParser has more usages.
#![allow(dead_code)]

#[derive(pest_derive::Parser)]
#[grammar = "text_macro.pest"]
pub struct TextMacroDocumentParser;

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser as _;

    macro_rules! assert_all_rule {
        ($rule:expr, $in:expr) => {
            for input in $in {
                let len = match TextMacroDocumentParser::parse($rule, input) {
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
                TextMacroDocumentParser::parse($rule, input).is_err()
                    || TextMacroDocumentParser::parse($rule, input)
                        .unwrap()
                        .last()
                        .unwrap()
                        .as_span()
                        .end()
                        != input.len()
            }));
        };
    }

    #[test]
    fn parse_accepted_idents() {
        let inputs = &["_", "_1", "a", "_a", "abc1234"];

        assert_all_rule!(Rule::ident, inputs);
    }

    #[test]
    fn parse_rejected_idents() {
        let inputs = &["1", "💩"];

        assert_all_not_rule!(Rule::ident, inputs);
    }

    #[test]
    fn parse_accepted_expression_placeholders() {
        let inputs = &["{{% abc %}}", "{{% 123 %}}"];

        assert_all_rule!(Rule::placeholder, inputs);
    }

    #[test]
    fn parse_rejected_expression_placeholders() {
        let inputs = &["{{%%}}", "{{% %}}", "{{%", "{{% }"];

        assert_all_not_rule!(Rule::placeholder, inputs);
    }

    #[test]
    fn parse_accepted_variable_placeholders() {
        let inputs = &[
            "{$var {%abc%}}",
            "{$var {% abc %}}",
            "{$var {% 123 %}}",
            "{$var {% 1 + 2 + b %}}",
            "{$var {% x + y / z %}}",
            "{$var_long {% x + 2 + 2d2 %}}",
        ];

        assert_all_rule!(Rule::placeholder, inputs);
    }

    #[test]
    fn parse_reject_variable_placeholders() {
        let inputs = &[
            "{$💩 {%abc%}}",
            "{$var {% abc",
            "{$var abc %}",
            "{$var {% {$var {% x %}} %}}",
            "{$var abc}",
        ];

        assert_all_not_rule!(Rule::placeholder, inputs);
    }

    #[test]
    fn parse_accepted_text_macro_links() {
        let inputs = &["[Test](#test)", "[Test Test](#test_test)"];

        assert_all_rule!(Rule::text_macro_link, inputs);
    }

    #[test]
    fn parse_rejected_text_macro_links() {
        let inputs = &[
            "[Test(test)",
            "[Test Test]($test_test)",
            "[Test Test] test)",
            "[Test Test]",
            "[Test Test] (test",
        ];

        assert_all_not_rule!(Rule::text_macro_link, inputs);
    }

    #[test]
    fn parse_accepted_text_macro_links_with_options() {
        let inputs = &[
            r#"[Test]("test": #test)"#,
            r#"[Test]("test": #test, "test2": #test2)"#,
            r#"[Test]("test": #test, "test2": #test2, "test test": #test3)"#,
        ];

        assert_all_rule!(Rule::text_macro_link, inputs);
    }

    #[test]
    fn parse_rejected_text_macro_links_with_options() {
        let inputs = &[
            r#"[Test]("test": )"#,
            r#"[Test]("test": #)"#,
            r#"[Test]("test": #2)"#,
            r#"[Test]("test": #t2, "test": #)"#,
            r#"[Test]("test": #t2, "test": #2)"#,
            r#"[Test]("test": #t2 "test": #t3)"#,
            r#"[Test]("test: #t2)"#,
            r#"[Test](test: #t2)"#,
            r#"[Test]("test": #t2, "test")"#,
        ];

        assert_all_not_rule!(Rule::text_macro_link, inputs);
    }

    #[test]
    fn parse_rejected_simple_macros() {
        let inputs = &[
            "Attack *{{% 1d20 + $self.strength %}}",
            "Attack *{{% 1d20 + $self.strength %}*",
            "Attack *{ 1d20 + $self.strength}*",
        ];

        assert_all_not_rule!(Rule::text_macro_document, inputs);
    }

    #[test]
    fn parse_accepted_complex_macros() {
        let inputs = &[
            include_str!("../../test/data/nonsense.txt"),
            include_str!("../../test/data/more_nonsense.txt"),
            include_str!("../../test/data/long_sword_basic_attack.txt"),
            include_str!("../../test/data/long_sword_multiple_attack.txt"),
            include_str!("../../test/data/eblast.txt"),
        ];

        assert_all_rule!(Rule::text_macro_document, inputs);
    }
}
