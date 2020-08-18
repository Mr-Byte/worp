use crate::syntax::span::Span;
use logos::Logos;
use std::iter::Iterator;

pub struct TokenIterator<'a> {
    inner: Box<dyn Iterator<Item = Token<'a>> + 'a>,
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
    span: Span,
    slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn tokenize(input: &'a str) -> TokenIterator<'a> {
        let inner = Box::new(TokenKind::lexer(input).spanned().map(move |(kind, span)| Token {
            kind,
            span: span.clone().into(),
            slice: &input[span],
        })) as Box<dyn Iterator<Item = Token<'a>>>;

        TokenIterator { inner }
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn slice(&self) -> &str {
        self.slice
    }

    pub const fn start_of_input() -> Token<'a> {
        Self {
            kind: TokenKind::StartOfInput,
            span: Span::new(0..0),
            slice: "",
        }
    }

    pub const fn end_of_input() -> Token<'a> {
        Self {
            kind: TokenKind::EndOfInput,
            span: Span::new(0..0),
            slice: "",
        }
    }

    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    pub fn is_any_kind(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self.kind)
    }
}

impl TokenKind {
    pub fn is_reserved(&self) -> bool {
        use TokenKind::*;

        matches!(
            self,
            While
                | Do
                | Loop
                | For
                | Break
                | Continue
                | Return
                | Yield
                | Function
                | Const
                | Match
                | Trait
                | In
                | Operator
                | Static
                | Class
                | Struct
                | Type
                | TypeOf
                | InstanceOf
                | Enum
                | Virtual
                | Override
                | Abstract
                | Final
                | Impl
                | Import
                | From
        )
    }
}

#[derive(Logos, Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    StartOfInput,
    // End of input.
    EndOfInput,
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
    #[regex("[_a-ce-zA-Z][_a-zA-Z0-9]*")]
    Identifier,
    #[regex(r#""((?:[^"\\]|\\.)*)""#)]
    String,
    #[regex("[0-9]+")]
    Integer,
    #[regex(r"[0-9]+\.[0-9]+")]
    Float,
    // Keywords
    #[token("object")]
    Object,
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
