// TODO: Remove this once TextMacroDocumentParser has more usages.
#![allow(dead_code)]

#[derive(pest_derive::Parser)]
#[grammar = "text_macro.pest"]
pub struct TextMacroDocumentParser;

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser as _;

    #[test]
    fn parse_ident() -> Result<(), Box<dyn std::error::Error>> {
        TextMacroDocumentParser::parse(Rule::ident, "_abcdef123")?;

        Ok(())
    }

    #[test]
    fn expression_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        TextMacroDocumentParser::parse(Rule::text_macro_document, "{{ abc }}")?;

        Ok(())
    }

    #[test]
    fn sub_macro_decl_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        if let Err(err) = TextMacroDocumentParser::parse(Rule::text_macro_document, "{#macro {% abc %}}") {
            eprintln!("{}", err);
            return Err(err)?;
        }

        Ok(())
    }

    #[test]
    fn complex_sub_macro_decl_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        let complex = "\
        {\n\
            #macro {%\n\
                *test*\n\
                {{1d20+10}}\n\
                % % % % % %\n\
            %}\n\
        }
        ";

        if let Err(err) = TextMacroDocumentParser::parse(Rule::text_macro_document, complex) {
            println!("{}", err);
            panic!("Test failed.")
        }

        Ok(())
    }

    #[test]
    fn variable_decl_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        if let Err(err) = TextMacroDocumentParser::parse(Rule::text_macro_document, "{$var {abc}}") {
            eprintln!("{}", err);
            return Err(err)?;
        }

        Ok(())
    }

    #[test]
    fn macro_link_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        if let Err(err) = TextMacroDocumentParser::parse(Rule::text_macro_document, "[Test](#test)") {
            eprintln!("{}", err);
            return Err(err)?;
        }

        Ok(())
    }

    #[test]
    fn macro_link_with_options_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        if let Err(err) = TextMacroDocumentParser::parse(Rule::text_macro_document, r#"[Test](#test > "test" #test2 > "test2")"#) {
            eprintln!("{}", err);
            return Err(err)?;
        }

        Ok(())
    }

    #[test]
    fn text_macro_document_parse_succeeds() {
        let input = "\
        raw text\n\
        with newlines\n\
        abc *abc {{55+5}}* more raw text {#test {%a%}}\n\
        {{8d6}}\n\
        \\* \n\
        // Commented section\n\
        [Test Macro](#test)\n\
        {$var {8d6}}\n\
        $var\n\
        #test\n\
        *test ~test _test_~*";

        if let Err(err) = TextMacroDocumentParser::parse(Rule::text_macro_document, input) {
            println!("{}", err);
            panic!("Test failed.")
        }
    }
}
