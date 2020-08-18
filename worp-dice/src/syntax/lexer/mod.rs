mod token;

pub use token::*;

#[cfg(test)]
pub mod test {
    use super::*;

    macro_rules! assert_next_token {
        ($tokens:expr, $token:pat) => {
            matches!(
                $tokens.next(),
                Some($crate::syntax::lexer::Token { kind: $token, .. })
            )
        };
    }

    #[test]
    fn tokenize_delimeters() {
        let delimeters = "( ) { } [ ] ; : ,";
        let mut tokens = Token::tokenize(delimeters);

        assert_next_token!(tokens, TokenKind::LeftParen);
        assert_next_token!(tokens, TokenKind::RightParen);
        assert_next_token!(tokens, TokenKind::LeftCurly);
        assert_next_token!(tokens, TokenKind::RightCurly);
        assert_next_token!(tokens, TokenKind::LeftSquare);
        assert_next_token!(tokens, TokenKind::RightSquare);
        assert_next_token!(tokens, TokenKind::Semicolon);
        assert_next_token!(tokens, TokenKind::Colon);
        assert_next_token!(tokens, TokenKind::Comma);
    }

    #[test]
    fn tokenize_operators() {
        let delimeters = ".. ..= -> => . ?. ?? % - + * / ! != == > >= < <= = d && ||";
        let mut tokens = Token::tokenize(delimeters);

        assert_next_token!(tokens, TokenKind::InclusiveRange);
        assert_next_token!(tokens, TokenKind::ExclusiveRange);
        assert_next_token!(tokens, TokenKind::Arrow);
        assert_next_token!(tokens, TokenKind::WideArrow);
        assert_next_token!(tokens, TokenKind::Dot);
        assert_next_token!(tokens, TokenKind::SafeDot);
        assert_next_token!(tokens, TokenKind::Coalesce);
        assert_next_token!(tokens, TokenKind::Minus);
        assert_next_token!(tokens, TokenKind::Remainder);
        assert_next_token!(tokens, TokenKind::Plus);
        assert_next_token!(tokens, TokenKind::Star);
        assert_next_token!(tokens, TokenKind::Slash);
        assert_next_token!(tokens, TokenKind::Not);
        assert_next_token!(tokens, TokenKind::NotEqual);
        assert_next_token!(tokens, TokenKind::Equal);
        assert_next_token!(tokens, TokenKind::Greater);
        assert_next_token!(tokens, TokenKind::GreaterEqual);
        assert_next_token!(tokens, TokenKind::Less);
        assert_next_token!(tokens, TokenKind::LessEqual);
        assert_next_token!(tokens, TokenKind::Assign);
        assert_next_token!(tokens, TokenKind::DiceRoll);
        assert_next_token!(tokens, TokenKind::LazyAnd);
        assert_next_token!(tokens, TokenKind::LazyOr);
    }

    #[test]
    fn tokenize_literals() {
        let delimeters = r#"1 -1 +1 1.0 -1.0 +1.0 abc _abc _123 "abc" "abc\"abc""#;
        let mut tokens = Token::tokenize(delimeters);

        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Float);
        assert_next_token!(tokens, TokenKind::Float);
        assert_next_token!(tokens, TokenKind::Float);
        assert_next_token!(tokens, TokenKind::Identifier);
        assert_next_token!(tokens, TokenKind::Identifier);
        assert_next_token!(tokens, TokenKind::Identifier);
        assert_next_token!(tokens, TokenKind::String);
        assert_next_token!(tokens, TokenKind::String);
    }

    #[test]
    fn tokenize_keywords() {
        let delimeters = "
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
            impl
            import
            from
        ";
        let mut tokens = Token::tokenize(delimeters);

        assert_next_token!(tokens, TokenKind::False);
        assert_next_token!(tokens, TokenKind::True);
        assert_next_token!(tokens, TokenKind::None);
        assert_next_token!(tokens, TokenKind::If);
        assert_next_token!(tokens, TokenKind::Else);
        assert_next_token!(tokens, TokenKind::While);
        assert_next_token!(tokens, TokenKind::Do);
        assert_next_token!(tokens, TokenKind::Loop);
        assert_next_token!(tokens, TokenKind::For);
        assert_next_token!(tokens, TokenKind::Break);
        assert_next_token!(tokens, TokenKind::Return);
        assert_next_token!(tokens, TokenKind::Yield);
        assert_next_token!(tokens, TokenKind::Continue);
        assert_next_token!(tokens, TokenKind::Let);
        assert_next_token!(tokens, TokenKind::Const);
        assert_next_token!(tokens, TokenKind::Match);
        assert_next_token!(tokens, TokenKind::Trait);
        assert_next_token!(tokens, TokenKind::In);
        assert_next_token!(tokens, TokenKind::Operator);
        assert_next_token!(tokens, TokenKind::Static);
        assert_next_token!(tokens, TokenKind::Class);
        assert_next_token!(tokens, TokenKind::Struct);
        assert_next_token!(tokens, TokenKind::Type);
        assert_next_token!(tokens, TokenKind::TypeOf);
        assert_next_token!(tokens, TokenKind::InstanceOf);
        assert_next_token!(tokens, TokenKind::Enum);
        assert_next_token!(tokens, TokenKind::Virtual);
        assert_next_token!(tokens, TokenKind::Override);
        assert_next_token!(tokens, TokenKind::Abstract);
        assert_next_token!(tokens, TokenKind::Final);
        assert_next_token!(tokens, TokenKind::Where);
        assert_next_token!(tokens, TokenKind::Impl);
        assert_next_token!(tokens, TokenKind::Import);
        assert_next_token!(tokens, TokenKind::From);
    }

    #[test]
    fn tokenize_errors() {
        let delimeters = r#"❤ @ \ ^"#;
        let mut tokens = Token::tokenize(delimeters);

        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
        assert_next_token!(tokens, TokenKind::Error);
    }

    #[test]
    fn tokenize_comment_yields_no_tokens() {
        let delimeters = r#"// test"#;
        let mut tokens = Token::tokenize(delimeters);

        assert!(tokens.next().is_none());
    }

    #[test]
    fn tokenize_token_followed_by_comment_yields_one_token() {
        let delimeters = r#"12 // test"#;
        let mut tokens = Token::tokenize(delimeters);

        assert_next_token!(tokens, TokenKind::Integer);
        assert!(tokens.next().is_none());
    }

    #[test]
    fn tokenize_token_followed_by_comment_followed_by_token_on_newline_yields_two_tokens() {
        let delimeters = r#"12 // test\n14"#;
        let mut tokens = Token::tokenize(delimeters);

        assert_next_token!(tokens, TokenKind::Integer);
        assert_next_token!(tokens, TokenKind::Integer);
        assert!(tokens.next().is_none());
    }
}
