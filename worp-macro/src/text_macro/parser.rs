#[derive(pest_derive::Parser)]
#[grammar = "text_macro.pest"]
pub struct TextMacroParser;

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser as _;

    #[test]
    fn parse_ident() -> Result<(), Box<dyn std::error::Error>> {
        TextMacroParser::parse(Rule::ident, "_abcdef123")?;

        Ok(())
    }

    #[test]
    fn expression_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        TextMacroParser::parse(Rule::text_macro, "{{ abc }}")?;

        Ok(())
    }

    #[test]
    fn sub_macro_decl_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        TextMacroParser::parse(Rule::text_macro, "{#macro [ abc ]}")?;

        Ok(())
    }

    #[test]
    fn variable_decl_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        if let Err(err) = TextMacroParser::parse(Rule::text_macro, "{$var {abc}}") {
            eprintln!("{}", err);
            return Err(err)?;
        }

        Ok(())
    }

    #[test]
    fn macro_link_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        if let Err(err) = TextMacroParser::parse(Rule::text_macro, "[Test](#test)") {
            eprintln!("{}", err);
            return Err(err)?;
        }

        Ok(())
    }

    #[test]
    fn text_macro_parse_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        let input = "\
        abc *abc {{55+5}}* {#test [a]}\n\
        {{8d6}}\n\
        \\* \n\
        // Commented section\n\
        [Test Macro](#test)\n\
        {$var {8d6}}\n\
        *test ~test _test_~*";

        if let Err(err) = TextMacroParser::parse(Rule::text_macro, input) {
            println!("{}", err);
            return Err(err)?;
        }

        Ok(())
    }
}
