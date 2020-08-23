use super::{
    error::SyntaxError,
    lexer::{Lexer, Token, TokenKind},
    Binary, BinaryOperator, Literal, SyntaxNode, SyntaxNodeId, SyntaxTree, Unary, UnaryOperator,
};
use id_arena::Arena;

type SyntaxNodeResult = Result<SyntaxNodeId, SyntaxError>;

struct ParserRule {
    prefix: Option<fn(&mut Parser) -> Result<SyntaxNodeId, SyntaxError>>,
    infix: Option<fn(&mut Parser, lhs: SyntaxNodeId) -> Result<SyntaxNodeId, SyntaxError>>,
    precedence: RulePrecedence,
}

impl ParserRule {
    fn new(
        prefix: Option<fn(&mut Parser) -> Result<SyntaxNodeId, SyntaxError>>,
        infix: Option<fn(&mut Parser, lhs: SyntaxNodeId) -> Result<SyntaxNodeId, SyntaxError>>,
        precedence: RulePrecedence,
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

impl ParserRule {
    fn for_token(token: &Token) -> Result<ParserRule, SyntaxError> {
        let rule = match token.kind {
            // Literals
            TokenKind::Integer(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Float(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::String(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::None => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::False => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::True => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Identifier(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),

            // Objects
            TokenKind::Object => ParserRule::new(Some(Parser::object), None, RulePrecedence::Object),
            TokenKind::RightCurly => ParserRule::new(None, None, RulePrecedence::None),
            TokenKind::LeftSquare => ParserRule::new(Some(Parser::list), None, RulePrecedence::Object),
            TokenKind::RightSquare => ParserRule::new(None, None, RulePrecedence::None),

            // Grouping
            TokenKind::LeftParen => ParserRule::new(Some(Parser::grouping), None, RulePrecedence::None),

            // Delimeters
            TokenKind::RightParen => ParserRule::new(None, None, RulePrecedence::None),
            TokenKind::Comma => ParserRule::new(None, None, RulePrecedence::None),
            TokenKind::Semicolon => ParserRule::new(None, None, RulePrecedence::None),
            TokenKind::Colon => ParserRule::new(None, None, RulePrecedence::None),

            // Control flow
            TokenKind::If => ParserRule::new(Some(Parser::if_expression), None, RulePrecedence::None),

            // Operators
            TokenKind::Coalesce => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Coalesce),
            TokenKind::ExclusiveRange => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Range),
            TokenKind::InclusiveRange => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Range),
            TokenKind::LazyAnd => ParserRule::new(None, Some(Parser::binary), RulePrecedence::And),
            TokenKind::LazyOr => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Or),
            TokenKind::Equal => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::NotEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Greater => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::GreaterEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Less => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::LessEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Star => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Slash => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Remainder => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Plus => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Term),
            TokenKind::Minus => ParserRule::new(Some(Parser::unary), Some(Parser::binary), RulePrecedence::Term),
            TokenKind::DiceRoll => ParserRule::new(Some(Parser::unary), Some(Parser::binary), RulePrecedence::DiceRoll),
            TokenKind::Not => ParserRule::new(Some(Parser::unary), None, RulePrecedence::Unary),

            // Setup reserved keywords and sequence with a parser that returns a friendly error.

            // End of input
            TokenKind::EndOfInput => ParserRule::new(None, None, RulePrecedence::None),
            _ => return Err(SyntaxError::UnexpectedToken(token.clone())),
        };

        Ok(rule)
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RulePrecedence {
    None,
    Assignment,
    Coalesce,
    Range,
    Or,
    And,
    Comparison,
    Term,
    Factor,
    DiceRoll,
    Unary,
    Call,
    Object,
    Primary,
}

impl RulePrecedence {
    fn increment(self) -> Self {
        match self {
            RulePrecedence::None => RulePrecedence::Assignment,
            RulePrecedence::Assignment => RulePrecedence::Coalesce,
            RulePrecedence::Coalesce => RulePrecedence::Range,
            RulePrecedence::Range => RulePrecedence::Or,
            RulePrecedence::Or => RulePrecedence::And,
            RulePrecedence::And => RulePrecedence::Comparison,
            RulePrecedence::Comparison => RulePrecedence::Term,
            RulePrecedence::Term => RulePrecedence::Factor,
            RulePrecedence::Factor => RulePrecedence::DiceRoll,
            RulePrecedence::DiceRoll => RulePrecedence::Unary,
            RulePrecedence::Unary => RulePrecedence::Call,
            RulePrecedence::Call => RulePrecedence::Object,
            RulePrecedence::Object => RulePrecedence::Primary,
            RulePrecedence::Primary => RulePrecedence::Primary,
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    arena: Arena<SyntaxNode>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let lexer = Lexer::from_str(input);
        let arena = Arena::new();

        Self { lexer, arena }
    }

    // TODO: Have this return a collection of parse errors.
    pub fn parse(mut self) -> Result<SyntaxTree, SyntaxError> {
        let root = self.expression()?;

        Ok(SyntaxTree::new(root, self.arena))
    }

    fn expression(&mut self) -> SyntaxNodeResult {
        self.parse_precedence(RulePrecedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: RulePrecedence) -> SyntaxNodeResult {
        let next_token = self.lexer.peek();
        let rule = ParserRule::for_token(&next_token)?;
        let mut node = rule.prefix.map(|prefix| prefix(self)).unwrap_or_else(|| todo!())?;

        loop {
            let next_token = self.lexer.peek();
            let rule = ParserRule::for_token(&next_token)?;

            if precedence > rule.precedence {
                break;
            }

            node = rule.infix.map(|infix| infix(self, node)).unwrap_or_else(|| todo!())?;
        }

        Ok(node)
    }

    fn if_expression(&mut self) -> SyntaxNodeResult {
        todo!()
    }

    fn binary(&mut self, lhs: SyntaxNodeId) -> SyntaxNodeResult {
        let token = self.lexer.next();
        let rule = ParserRule::for_token(&token)?;
        let rhs = self.parse_precedence(rule.precedence.increment())?;

        let operator = match token.kind {
            TokenKind::Coalesce => BinaryOperator::Coalesce,
            TokenKind::ExclusiveRange => BinaryOperator::RangeExclusive,
            TokenKind::InclusiveRange => BinaryOperator::RangeInclusive,
            TokenKind::LazyAnd => BinaryOperator::LogicalAnd,
            TokenKind::LazyOr => BinaryOperator::LogicalOr,
            TokenKind::Equal => BinaryOperator::Equals,
            TokenKind::NotEqual => BinaryOperator::NotEquals,
            TokenKind::Greater => BinaryOperator::GreaterThan,
            TokenKind::GreaterEqual => BinaryOperator::GreaterThanEquals,
            TokenKind::Less => BinaryOperator::LessThan,
            TokenKind::LessEqual => BinaryOperator::LessThanEquals,
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::Minus => BinaryOperator::Subtract,
            TokenKind::Star => BinaryOperator::Multiply,
            TokenKind::Slash => BinaryOperator::Divide,
            TokenKind::Remainder => BinaryOperator::Remainder,
            TokenKind::DiceRoll => BinaryOperator::DiceRoll,
            _ => unreachable!(),
        };

        let node = SyntaxNode::Binary(Binary(operator, lhs, rhs, token.span()));
        Ok(self.arena.alloc(node))
    }

    fn unary(&mut self) -> SyntaxNodeResult {
        let token = self.lexer.next();
        let child_node_id = self.parse_precedence(RulePrecedence::Unary)?;
        let operator = match token.kind {
            TokenKind::Minus => UnaryOperator::Negate,
            TokenKind::Not => UnaryOperator::Not,
            TokenKind::DiceRoll => UnaryOperator::DiceRoll,
            _ => unreachable!(),
        };
        let node = SyntaxNode::Unary(Unary(operator, child_node_id, token.span()));

        Ok(self.arena.alloc(node))
    }

    fn grouping(&mut self) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::LeftParen)?;
        let expression = self.expression()?;
        self.lexer.consume(TokenKind::RightParen)?;

        Ok(expression)
    }

    fn object(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Object)?.span();
        self.lexer.consume(TokenKind::LeftCurly)?;

        let mut properties = Vec::new();

        while self.lexer.peek().kind != TokenKind::RightCurly {
            let key = self.parse_precedence(RulePrecedence::Primary)?;
            self.lexer.consume(TokenKind::Colon)?;
            let value = self.parse_precedence(RulePrecedence::Assignment)?;

            if self.lexer.peek().kind == TokenKind::Comma {
                self.lexer.next();
            } else if self.lexer.peek().kind != TokenKind::RightCurly {
                return Err(SyntaxError::UnexpectedToken(self.lexer.next()));
            }

            properties.push((key, value));
        }

        let span_end = self.lexer.consume(TokenKind::RightCurly)?.span();

        let node = self
            .arena
            .alloc(SyntaxNode::Literal(Literal::Object(properties, span_start + span_end)));

        Ok(node)
    }

    fn list(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::LeftSquare)?.span();

        let mut values = Vec::new();

        while self.lexer.peek().kind != TokenKind::RightSquare {
            let value = self.parse_precedence(RulePrecedence::Assignment)?;

            if self.lexer.peek().kind == TokenKind::Comma {
                self.lexer.next();
            } else if self.lexer.peek().kind != TokenKind::RightSquare {
                return Err(SyntaxError::UnexpectedToken(self.lexer.next()));
            }

            values.push(value);
        }

        let span_end = self.lexer.consume(TokenKind::RightSquare)?.span();

        let node = self
            .arena
            .alloc(SyntaxNode::Literal(Literal::List(values, span_start + span_end)));

        Ok(node)
    }

    fn literal(&mut self) -> SyntaxNodeResult {
        let token = self.lexer.next();
        let span = token.span();
        let literal = match token.kind {
            TokenKind::Integer(value) => Literal::Integer(value, span),
            TokenKind::Float(value) => Literal::Float(value, span),
            TokenKind::String(value) => Literal::String(value, span),
            TokenKind::Identifier(value) => Literal::Identifier(value, span),
            TokenKind::False => Literal::Boolean(false, span),
            TokenKind::True => Literal::Boolean(true, span),
            TokenKind::None => Literal::None(span),
            _ => return Err(SyntaxError::UnexpectedToken(token.clone())),
        };
        let node = SyntaxNode::Literal(literal);

        Ok(self.arena.alloc(node))
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::syntax::{error::SyntaxError, Binary, BinaryOperator, Literal, SyntaxNode, Unary, UnaryOperator};

    #[test]
    fn test_parse_integer() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(root, Some(SyntaxNode::Literal(Literal::Integer(5, _)))));

        Ok(())
    }

    #[test]
    fn test_parse_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("-5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Unary(Unary(UnaryOperator::Negate, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus_with_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("-5 - 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_right() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5 * 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_left() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * 5 - 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_grouping() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * (5 - 5)").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Multiply, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_unary_die() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("d8").parse()?;
        let root = syntax_tree.root();

        println!("{}", syntax_tree);

        assert!(matches!(
            root,
            Some(SyntaxNode::Unary(Unary(UnaryOperator::DiceRoll, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_dice() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("6d8").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::DiceRoll, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_object_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("object { x: 50, y: 30 }").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(root, Some(SyntaxNode::Literal(Literal::Object(_, _)))));

        Ok(())
    }

    #[test]
    fn test_parse_list_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("[x, y, 1, 1*2, object {}]").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(root, Some(SyntaxNode::Literal(Literal::List(_, _)))));

        Ok(())
    }
}
