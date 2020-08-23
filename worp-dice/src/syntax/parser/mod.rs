use super::{
    lexer::{Lexer, TokenKind},
    Binary, BinaryOperator, Literal, SyntaxNode, SyntaxNodeId, SyntaxTree, Unary, UnaryOperator,
};
use id_arena::Arena;

struct ParserRule {
    prefix: Option<fn(&mut Parser) -> Result<SyntaxNodeId, ()>>,
    infix: Option<fn(&mut Parser, lhs: SyntaxNodeId) -> Result<SyntaxNodeId, ()>>,
    precedence: RulePrecedence,
}

impl ParserRule {
    fn new(
        prefix: Option<fn(&mut Parser) -> Result<SyntaxNodeId, ()>>,
        infix: Option<fn(&mut Parser, lhs: SyntaxNodeId) -> Result<SyntaxNodeId, ()>>,
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
    fn for_token(kind: &TokenKind) -> Result<ParserRule, ()> {
        let rule = match kind {
            // Literals
            TokenKind::Integer(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),

            // Grouping
            TokenKind::LeftParen => ParserRule::new(None, None, RulePrecedence::Primary),

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

            // End of input
            TokenKind::EndOfInput => ParserRule::new(None, None, RulePrecedence::None),
            _ => return Err(()),
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
            RulePrecedence::Call => RulePrecedence::Primary,
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
    pub fn parse(mut self) -> Result<SyntaxTree, ()> {
        let root = self.expression()?;

        Ok(SyntaxTree::new(root, self.arena))
    }

    fn expression(&mut self) -> Result<SyntaxNodeId, ()> {
        self.parse_precedence(RulePrecedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: RulePrecedence) -> Result<SyntaxNodeId, ()> {
        let next_token = self.lexer.peek();
        let rule = ParserRule::for_token(&next_token.kind)?;
        let mut node = rule.prefix.map(|prefix| prefix(self)).unwrap_or_else(|| todo!())?;

        loop {
            let next_token = self.lexer.peek();
            let rule = ParserRule::for_token(&next_token.kind)?;

            if precedence > rule.precedence {
                break;
            }

            node = rule.infix.map(|infix| infix(self, node)).unwrap_or_else(|| todo!())?;
        }

        Ok(node)
    }

    fn binary(&mut self, lhs: SyntaxNodeId) -> Result<SyntaxNodeId, ()> {
        let token = self.lexer.next();
        let rule = ParserRule::for_token(&token.kind)?;
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

    fn unary(&mut self) -> Result<SyntaxNodeId, ()> {
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

    fn literal(&mut self) -> Result<SyntaxNodeId, ()> {
        let token = self.lexer.next();
        let node_id = match token.kind {
            TokenKind::Integer(value) => {
                let node = SyntaxNode::Literal(Literal::Integer(value, token.span()));
                self.arena.alloc(node)
            }
            _ => return Err(()),
        };

        Ok(node_id)
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::syntax::{Binary, BinaryOperator, Literal, SyntaxNode, Unary, UnaryOperator};

    #[test]
    fn test_parse_integer() -> Result<(), ()> {
        let syntax_tree = Parser::new("5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(root, Some(SyntaxNode::Literal(Literal::Integer(5, _)))));

        Ok(())
    }

    #[test]
    fn test_parse_unary_minus() -> Result<(), ()> {
        let syntax_tree = Parser::new("-5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Unary(Unary(UnaryOperator::Negate, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus() -> Result<(), ()> {
        let syntax_tree = Parser::new("5 - 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus_with_unary_minus() -> Result<(), ()> {
        let syntax_tree = Parser::new("-5 - 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_right() -> Result<(), ()> {
        let syntax_tree = Parser::new("5 - 5 * 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_left() -> Result<(), ()> {
        let syntax_tree = Parser::new("5 * 5 - 5").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
        ));

        Ok(())
    }

    #[test]
    fn test_parse_unary_die() -> Result<(), ()> {
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
    fn test_parse_binary_dice() -> Result<(), ()> {
        let syntax_tree = Parser::new("6d8").parse()?;
        let root = syntax_tree.root();

        assert!(matches!(
            root,
            Some(SyntaxNode::Binary(Binary(BinaryOperator::DiceRoll, _, _, _)))
        ));

        Ok(())
    }
}
