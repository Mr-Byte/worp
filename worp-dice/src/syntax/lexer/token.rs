use crate::runtime::core::span::Span;
use logos::Logos;
use std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
        TokenKind::lexer(input).spanned().map(move |(kind, span)| Token {
            kind,
            span: span.into(),
        })
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub const fn end_of_input() -> Token {
        Self {
            kind: TokenKind::EndOfInput,
            span: Span::new(0..0),
        }
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum TokenKind {
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
    #[token("export")]
    Export,

    // Literals,
    #[regex("[_a-ce-zA-Z][_a-zA-Z0-9]*", |lex| lex.slice().to_owned())]
    Identifier(String),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),
    #[regex(r#""((?:[^"\\]|\\.)*)""#, |lex| lex.slice().to_owned())]
    String(String),

    #[error]
    #[regex(r"[ \t\r\n\f]+|//[^\r\n]+", logos::skip)]
    Error,
}
