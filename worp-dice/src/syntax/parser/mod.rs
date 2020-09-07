use super::{
    error::SyntaxError,
    lexer::{Lexer, Token, TokenKind},
    Assignment, AssignmentOperator, Binary, BinaryOperator, Block, Break, Continue, Discard, IfExpression, LitBool,
    LitFloat, LitIdent, LitInt, LitList, LitNone, LitObject, LitString, LitUnit, Literal, SyntaxNode, SyntaxNodeId,
    SyntaxTree, Unary, UnaryOperator, VariableDeclaration, WhileLoop,
};
use crate::runtime::core::Span;
use id_arena::Arena;

type SyntaxNodeResult = Result<SyntaxNodeId, SyntaxError>;

type PrefixParser = fn(&mut Parser, can_assign: bool) -> Result<SyntaxNodeId, SyntaxError>;
type InfixParser = fn(&mut Parser, lhs: SyntaxNodeId, span: Span) -> Result<SyntaxNodeId, SyntaxError>;

#[derive(Default)]
struct ParserRule {
    prefix: Option<PrefixParser>,
    infix: Option<InfixParser>,
    precedence: RulePrecedence,
}

impl ParserRule {
    fn new(prefix: Option<PrefixParser>, infix: Option<InfixParser>, precedence: RulePrecedence) -> Self {
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
            // Empty rules
            TokenKind::RightSquare => ParserRule::default(),
            TokenKind::RightParen => ParserRule::default(),
            TokenKind::RightCurly => ParserRule::default(),
            TokenKind::Semicolon => ParserRule::default(),
            TokenKind::Comma => ParserRule::default(),
            TokenKind::Colon => ParserRule::default(),
            TokenKind::Assign => ParserRule::default(),
            TokenKind::MulAssign => ParserRule::default(),
            TokenKind::DivAssign => ParserRule::default(),
            TokenKind::AddAssign => ParserRule::default(),
            TokenKind::SubAssign => ParserRule::default(),

            // Literals
            TokenKind::Integer(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Float(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::String(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::None => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::False => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::True => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Identifier(_) => ParserRule::new(Some(Parser::variable), None, RulePrecedence::Primary),

            //
            TokenKind::If => ParserRule::new(Some(Parser::if_expression), None, RulePrecedence::None),

            // Objects
            TokenKind::Object => ParserRule::new(Some(Parser::object), None, RulePrecedence::Object),
            TokenKind::LeftSquare => ParserRule::new(Some(Parser::list), None, RulePrecedence::Object),

            // Grouping
            TokenKind::LeftParen => ParserRule::new(Some(Parser::grouping), None, RulePrecedence::Primary),

            // Control flow
            // TokenKind::If => ParserRule::new(Some(Parser::if_expression), None, RulePrecedence::None),
            // TokenKind::While => ParserRule::new(Some(Parser::while_expression), None, RulePrecedence::None),

            // Block expressions
            TokenKind::LeftCurly => ParserRule::new(Some(Parser::block_expression), None, RulePrecedence::None),

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

impl Default for RulePrecedence {
    fn default() -> Self {
        Self::None
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
        let root = self.expression_sequence()?;

        Ok(SyntaxTree::new(root, self.arena))
    }

    fn expression_sequence(&mut self) -> SyntaxNodeResult {
        let mut items = Vec::new();
        let mut next_token = self.lexer.peek();
        let span_start = next_token.span();

        loop {
            if next_token.kind == TokenKind::RightCurly || next_token.kind == TokenKind::EndOfInput {
                break;
            }

            if next_token.kind == TokenKind::If {
                let expression = self.if_expression(false)?;
                items.push(expression);
                next_token = self.lexer.peek();
            } else if next_token.kind == TokenKind::While {
                let expression = self.while_statement()?;
                items.push(expression);
                next_token = self.lexer.peek();
            } else if next_token.kind == TokenKind::Let {
                let expression = self.variable_decl()?;
                items.push(expression);
                next_token = self.lexer.peek();
            } else if next_token.kind == TokenKind::Break || next_token.kind == TokenKind::Continue {
                let expression = self.control_flow()?;
                items.push(expression);
                next_token = self.lexer.peek();
            } else {
                let expression = self.expression()?;

                items.push(expression);
                next_token = self.lexer.peek();

                if next_token.kind == TokenKind::Semicolon {
                    let semi_token = self.lexer.consume(TokenKind::Semicolon)?;
                    let discard = SyntaxNode::Discard(Discard(semi_token.span()));
                    items.push(self.arena.alloc(discard));

                    next_token = self.lexer.peek();
                }
            }
        }

        let span_end = next_token.span();
        let node = SyntaxNode::Block(Block(items, span_start + span_end));

        Ok(self.arena.alloc(node))
    }

    fn expression(&mut self) -> SyntaxNodeResult {
        self.parse_precedence(RulePrecedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: RulePrecedence) -> SyntaxNodeResult {
        let next_token = self.lexer.peek();
        let rule = ParserRule::for_token(&next_token)?;
        let mut node = rule
            .prefix
            .map(|prefix| prefix(self, precedence <= RulePrecedence::Assignment))
            .unwrap_or_else(|| Err(SyntaxError::UnexpectedToken(next_token.clone())))?;

        loop {
            let span_start = next_token.span();
            let next_token = self.lexer.peek();
            let rule = ParserRule::for_token(&next_token)?;

            if precedence > rule.precedence {
                break;
            }

            node = rule
                .infix
                .map(|infix| infix(self, node, span_start))
                .unwrap_or_else(|| Err(SyntaxError::UnexpectedToken(next_token)))?;
        }

        Ok(node)
    }

    fn if_expression(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::If)?.span();
        let condition = self.expression()?;
        let primary = self.block_expression(false)?;
        let secondary = if self.lexer.peek().kind == TokenKind::Else {
            self.lexer.consume(TokenKind::Else)?;

            match self.lexer.peek().kind {
                TokenKind::If => Some(self.if_expression(false)?),
                TokenKind::LeftCurly => Some(self.block_expression(false)?),
                _ => None,
            }
        } else {
            None
        };
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::IfExpression(IfExpression(condition, primary, secondary, span_start + span_end));

        Ok(self.arena.alloc(node))
    }

    fn while_statement(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::While)?.span();
        let condition = self.expression()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();

        let node = SyntaxNode::WhileLoop(WhileLoop(condition, body, span_start + span_end));

        Ok(self.arena.alloc(node))
    }

    fn control_flow(&mut self) -> SyntaxNodeResult {
        let token = self.lexer.next();

        let node = match token.kind {
            TokenKind::Break => SyntaxNode::Break(Break(token.span())),
            TokenKind::Continue => SyntaxNode::Continue(Continue(token.span())),
            _ => return Err(SyntaxError::UnexpectedToken(token)),
        };

        self.lexer.consume(TokenKind::Semicolon)?;

        Ok(self.arena.alloc(node))
    }

    fn block_expression(&mut self, _: bool) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::LeftCurly)?;
        let expressions = self.expression_sequence()?;
        self.lexer.consume(TokenKind::RightCurly)?;

        Ok(expressions)
    }

    fn variable(&mut self, can_assign: bool) -> SyntaxNodeResult {
        let next_token = self.lexer.next();
        let span_start = next_token.span();
        let mut expression = if let TokenKind::Identifier(name) = next_token.kind {
            self.arena
                .alloc(SyntaxNode::Literal(Literal::Ident(LitIdent(name, span_start.clone()))))
        } else {
            return Err(SyntaxError::UnexpectedToken(next_token));
        };

        let next_token_kind = self.lexer.peek().kind;
        let is_assignent = matches!(
            next_token_kind,
            TokenKind::Assign
                | TokenKind::MulAssign
                | TokenKind::DivAssign
                | TokenKind::AddAssign
                | TokenKind::SubAssign
        );

        if can_assign && is_assignent {
            let kind = self
                .lexer
                .consume_one_of(&[
                    TokenKind::Assign,
                    TokenKind::MulAssign,
                    TokenKind::DivAssign,
                    TokenKind::AddAssign,
                    TokenKind::SubAssign,
                ])?
                .kind;

            let value = self.expression()?;
            let span_end = self.lexer.current().span();
            let op = match kind {
                TokenKind::Assign => AssignmentOperator::Assignment,
                TokenKind::MulAssign => AssignmentOperator::MulAssignment,
                TokenKind::DivAssign => AssignmentOperator::DivAssignment,
                TokenKind::AddAssign => AssignmentOperator::AddAssignment,
                TokenKind::SubAssign => AssignmentOperator::SubAssignment,
                _ => todo!(),
            };

            expression = self.arena.alloc(SyntaxNode::Assignment(Assignment(
                op,
                expression,
                value,
                span_start + span_end,
            )));
        }

        Ok(expression)
    }

    fn variable_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Let)?.span();

        let is_mutable = if self.lexer.peek().kind == TokenKind::Mut {
            self.lexer.consume(TokenKind::Mut)?;
            true
        } else {
            false
        };

        let token = self.lexer.next();
        let name = if let TokenKind::Identifier(name) = token.kind {
            name
        } else {
            return Err(SyntaxError::UnexpectedToken(token));
        };

        self.lexer.consume(TokenKind::Assign)?;
        let expression = self.expression()?;
        let span_end = self.lexer.current().span();

        self.lexer.consume(TokenKind::Semicolon)?;

        let node =
            SyntaxNode::VariableDeclaration(VariableDeclaration(name, is_mutable, expression, span_start + span_end));

        Ok(self.arena.alloc(node))
    }

    fn binary(&mut self, lhs: SyntaxNodeId, span_start: Span) -> SyntaxNodeResult {
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

        let node = SyntaxNode::Binary(Binary(operator, lhs, rhs, span_start + token.span()));
        Ok(self.arena.alloc(node))
    }

    fn unary(&mut self, _: bool) -> SyntaxNodeResult {
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

    fn grouping(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::LeftParen)?.span();

        if self.lexer.peek().kind == TokenKind::RightParen {
            let span_end = self.lexer.consume(TokenKind::RightParen)?.span();

            let node = SyntaxNode::Literal(Literal::Unit(LitUnit(span_start + span_end)));
            Ok(self.arena.alloc(node))
        } else {
            // TODO: Inject the remainder of the span?
            let expression = self.expression()?;
            // TODO: Detect trailing commas and produce a tuple instead of a group?
            // How to support single-element tuples?
            // Do like rust and require a singular trailing comma for single element tuples!
            self.lexer.consume(TokenKind::RightParen)?;

            Ok(expression)
        }
    }

    fn object(&mut self, _: bool) -> SyntaxNodeResult {
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

        let node = self.arena.alloc(SyntaxNode::Literal(Literal::Object(LitObject(
            properties,
            span_start + span_end,
        ))));

        Ok(node)
    }

    fn list(&mut self, _: bool) -> SyntaxNodeResult {
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

        let node = self.arena.alloc(SyntaxNode::Literal(Literal::List(LitList(
            values,
            span_start + span_end,
        ))));

        Ok(node)
    }

    fn literal(&mut self, _: bool) -> SyntaxNodeResult {
        let token = self.lexer.next();
        let span = token.span();
        let literal = match token.kind {
            TokenKind::Integer(value) => Literal::Integer(LitInt(value, span)),
            TokenKind::Float(value) => Literal::Float(LitFloat(value, span)),
            TokenKind::String(value) => Literal::String(LitString(value.trim_matches('"').to_owned(), span)),
            TokenKind::False => Literal::Bool(LitBool(false, span)),
            TokenKind::True => Literal::Bool(LitBool(true, span)),
            TokenKind::None => Literal::None(LitNone(span)),
            _ => return Err(SyntaxError::UnexpectedToken(token.clone())),
        };
        let node = SyntaxNode::Literal(literal);

        Ok(self.arena.alloc(node))
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::syntax::{
        error::SyntaxError, Binary, BinaryOperator, Block, LitInt, Literal, SyntaxNode, Unary, UnaryOperator,
    };

    #[test]
    fn test_parse_integer() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Literal(Literal::Integer(LitInt(5, _))))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("-5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Unary(Unary(UnaryOperator::Negate, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus_with_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - -5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_right() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5 * 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_left() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * 5 - 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary(BinaryOperator::Subtract, _, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_grouping() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * (5 - 5)").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary(BinaryOperator::Multiply, _, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_unary_die() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("d8").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Unary(Unary(UnaryOperator::DiceRoll, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_dice() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("6d8").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary(BinaryOperator::DiceRoll, _, _, _)))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_object_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("object { x: 50, y: 30 }").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(node, Some(SyntaxNode::Literal(Literal::Object(_)))));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_list_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("[x, y, 1, 1*2, object {}]").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block(block, _)) = root {
            let node = syntax_tree.get(*block.first().unwrap());

            assert!(matches!(node, Some(SyntaxNode::Literal(Literal::List(_)))));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }
}
