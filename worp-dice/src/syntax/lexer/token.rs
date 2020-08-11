use crate::syntax::span::Span;
use logos::Logos;
use std::iter::{Iterator, Peekable};

pub struct TokenIterator<'a> {
    inner: Peekable<Box<dyn Iterator<Item = Token<'a>> + 'a>>,
}

impl<'a> TokenIterator<'a> {
    pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
        self.inner.peek()
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Span,
    slice: &'a str,
}

impl Token<'_> {
    pub fn tokenize(input: &str) -> TokenIterator<'_> {
        let inner = (Box::new(TokenKind::lexer(input).spanned().map(move |(kind, span)| Token {
            kind: kind.clone(),
            span: span.clone().into(),
            slice: &input[span],
        })) as Box<dyn Iterator<Item = Token<'_>> + '_>)
            .peekable();

        TokenIterator { inner }
    }

    pub fn slice(&self) -> &str {
        self.slice
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Delimeters
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    // Operators
    #[token("..")]
    ExclusiveRange,
    #[token("..=")]
    InclusiveRange,
    #[token("->")]
    Arrow,
    #[token("=>")]
    WideArrow,
    #[token(".")]
    Dot,
    #[token("?.")]
    SafeDot,
    #[token("??")]
    Coalesce,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("%")]
    Remainder,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("!")]
    Not,
    #[token("!=")]
    NotEqual,
    #[token("==")]
    Equal,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token("=")]
    Assign,
    #[token("d")]
    DiceRoll,
    #[token("&&")]
    LazyAnd,
    #[token("||")]
    LazyOr,
    // Literals,
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier,
    #[regex(r#""((?:[^"\\]|\\.)*)""#)]
    String,
    #[regex("[+-]?[0-9]+")]
    Integer,
    #[regex(r"[+-]?[0-9]+\.[0-9]+")]
    Float,
    // Keywords
    #[token("false")]
    False,
    #[token("true")]
    True,
    #[token("none")]
    None,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("loop")]
    Loop,
    #[token("for")]
    For,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    #[token("yield")]
    Yield,
    #[token("fn")]
    Function,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("match")]
    Match,
    #[token("trait")]
    Trait,
    #[token("in")]
    In,
    #[token("operator")]
    Operator,
    #[token("static")]
    Static,
    #[token("class")]
    Class,
    #[token("struct")]
    Struct,
    #[token("type")]
    Type,
    #[token("typeof")]
    TypeOf,
    #[token("instanceof")]
    InstanceOf,
    #[token("enum")]
    Enum,
    #[token("virtual")]
    Virtual,
    #[token("override")]
    Override,
    #[token("abstract")]
    Abstract,
    #[token("final")]
    Final,
    #[token("where")]
    Where,
    #[token("impl")]
    Impl,
    #[token("import")]
    Import,
    #[token("from")]
    From,

    #[error]
    #[regex(r"[ \t\r\n\f]+|//[^\r\n]+", logos::skip)]
    Error,
}
