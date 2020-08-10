mod token;

pub use token::*;

#[cfg(test)]
pub mod test {
    use super::*;
    use logos::Logos as _;

    macro_rules! assert_next_token {
        ($tokens:expr, $token:pat) => {
            matches!($tokens.next(), Some($token))
        };
    }

    #[test]
    fn tokenize_delimeters() {
        let delimeters = "( ) { } [ ] ; : ,";
        let mut tokens = Token::lexer(delimeters);

        assert_next_token!(tokens, Token::LeftParen);
        assert_next_token!(tokens, Token::RightParen);
        assert_next_token!(tokens, Token::LeftCurly);
        assert_next_token!(tokens, Token::RightCurly);
        assert_next_token!(tokens, Token::LeftSquare);
        assert_next_token!(tokens, Token::RightSquare);
        assert_next_token!(tokens, Token::Semicolon);
        assert_next_token!(tokens, Token::Colon);
        assert_next_token!(tokens, Token::Comma);
    }

    #[test]
    fn tokenize_operators() {
        let delimeters = "-> => . ?. ?? % - + * / ! != == > >= < <= = d && ||";
        let mut tokens = Token::lexer(delimeters);

        assert_next_token!(tokens, Token::Arrow);
        assert_next_token!(tokens, Token::WideArrow);
        assert_next_token!(tokens, Token::Dot);
        assert_next_token!(tokens, Token::SafeDot);
        assert_next_token!(tokens, Token::Coalesce);
        assert_next_token!(tokens, Token::Minus);
        assert_next_token!(tokens, Token::Remainder);
        assert_next_token!(tokens, Token::Plus);
        assert_next_token!(tokens, Token::Star);
        assert_next_token!(tokens, Token::Slash);
        assert_next_token!(tokens, Token::Not);
        assert_next_token!(tokens, Token::NotEqual);
        assert_next_token!(tokens, Token::Equal);
        assert_next_token!(tokens, Token::Greater);
        assert_next_token!(tokens, Token::GreaterEqual);
        assert_next_token!(tokens, Token::Less);
        assert_next_token!(tokens, Token::LessEqual);
        assert_next_token!(tokens, Token::Assign);
        assert_next_token!(tokens, Token::DiceRoll);
        assert_next_token!(tokens, Token::LazyAnd);
        assert_next_token!(tokens, Token::LazyOr);
    }

    #[test]
    fn tokenize_literals() {
        let delimeters = r#"1 -1 +1 1.0 -1.0 +1.0 abc _abc _123 "abc" "abc\"abc""#;
        let mut tokens = Token::lexer(delimeters);

        assert_next_token!(tokens, Token::Integer);
        assert_next_token!(tokens, Token::Integer);
        assert_next_token!(tokens, Token::Integer);
        assert_next_token!(tokens, Token::Float);
        assert_next_token!(tokens, Token::Float);
        assert_next_token!(tokens, Token::Float);
        assert_next_token!(tokens, Token::Identifier);
        assert_next_token!(tokens, Token::Identifier);
        assert_next_token!(tokens, Token::Identifier);
        assert_next_token!(tokens, Token::String);
        assert_next_token!(tokens, Token::String);
    }

    #[test]
    fn tokenize_keywords() {
        let delimeters = "\
            false
            true
            none
            if
            else
            while
            do
            loop
            for
            break
            continue
            return
            yield
            fn
            let
            const
            match
            trait
            in
            operator
            static
            class
            struct
            type
            typeof
            instanceof
            enum
            virtual
            override
            abstract
            final
            where
        ";
        let mut tokens = Token::lexer(delimeters);

        assert_next_token!(tokens, Token::False);
        assert_next_token!(tokens, Token::True);
        assert_next_token!(tokens, Token::None);
        assert_next_token!(tokens, Token::If);
        assert_next_token!(tokens, Token::Else);
        assert_next_token!(tokens, Token::While);
        assert_next_token!(tokens, Token::Do);
        assert_next_token!(tokens, Token::Loop);
        assert_next_token!(tokens, Token::For);
        assert_next_token!(tokens, Token::Break);
        assert_next_token!(tokens, Token::Return);
        assert_next_token!(tokens, Token::Yield);
        assert_next_token!(tokens, Token::Continue);
        assert_next_token!(tokens, Token::Let);
        assert_next_token!(tokens, Token::Const);
        assert_next_token!(tokens, Token::Match);
        assert_next_token!(tokens, Token::Trait);
        assert_next_token!(tokens, Token::In);
        assert_next_token!(tokens, Token::Operator);
        assert_next_token!(tokens, Token::Static);
        assert_next_token!(tokens, Token::Class);
        assert_next_token!(tokens, Token::Struct);
        assert_next_token!(tokens, Token::Type);
        assert_next_token!(tokens, Token::TypeOf);
        assert_next_token!(tokens, Token::InstanceOf);
        assert_next_token!(tokens, Token::Enum);
        assert_next_token!(tokens, Token::Virtual);
        assert_next_token!(tokens, Token::Override);
        assert_next_token!(tokens, Token::Abstract);
        assert_next_token!(tokens, Token::Final);
        assert_next_token!(tokens, Token::Where);
    }

    #[test]
    fn tokenize_errors() {
        let delimeters = r#"❤ @ \ ^"#;
        let mut tokens = Token::lexer(delimeters);

        assert_next_token!(tokens, Token::Error);
        assert_next_token!(tokens, Token::Error);
        assert_next_token!(tokens, Token::Error);
        assert_next_token!(tokens, Token::Error);
    }
}
