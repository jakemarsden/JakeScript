use crate::ast::{self, *};
use crate::lexer::{self, *};
use crate::util::{IntoPeekableNth, PeekableNth};
use std::iter::Iterator;

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: PeekableNth<I>,
    constants: ConstantPool,
}

impl<I: Iterator<Item = char>> Parser<Tokens<Lexer<I>>> {
    pub fn for_lexer(source: Lexer<I>) -> Self {
        Self::for_tokens(source.tokens())
    }
}

impl<I: Iterator<Item = Element>> Parser<Tokens<I>> {
    pub fn for_elements(source: I) -> Self {
        Self::for_tokens(source.filter_map(Element::token))
    }
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn for_tokens(source: I) -> Self {
        Self {
            tokens: source.peekable_nth(),
            constants: ConstantPool::default(),
        }
    }

    pub fn execute(mut self) -> Program {
        let mut stmts = Vec::new();
        while let Some(stmt) = self.parse_statement() {
            stmts.push(stmt);
        }
        Program::new(Block::new(stmts), self.constants)
    }

    fn parse_block(&mut self) -> Block {
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::OpenBrace));
        let mut stmts = Vec::new();
        while !matches!(
            self.tokens.peek(),
            Some(Token::Punctuator(Punctuator::CloseBrace))
        ) {
            if let Some(stmt) = self.parse_statement() {
                stmts.push(stmt);
            } else {
                // Block not closed before end of input.
                break;
            }
        }
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::CloseBrace));
        Block::new(stmts)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.tokens.peek()? {
            Token::Keyword(Keyword::If) => self.parse_if_statement().map(Statement::IfStatement),
            Token::Keyword(Keyword::Function) => self
                .parse_function_declaration()
                .map(Statement::FunctionDeclaration),
            Token::Keyword(Keyword::For) => self.parse_for_loop().map(Statement::ForLoop),
            Token::Keyword(Keyword::While) => self.parse_while_loop().map(Statement::WhileLoop),

            token => {
                let stmt = match token {
                    Token::Keyword(Keyword::Assert) => {
                        self.parse_assertion().map(Statement::Assertion)
                    }
                    Token::Keyword(Keyword::Print | Keyword::PrintLn) => {
                        self.parse_print_statement().map(Statement::Print)
                    }
                    Token::Keyword(Keyword::Break) => {
                        self.parse_break_statement().map(Statement::Break)
                    }
                    Token::Keyword(Keyword::Continue) => {
                        self.parse_continue_statement().map(Statement::Continue)
                    }
                    Token::Keyword(Keyword::Return) => {
                        self.parse_return_statement().map(Statement::Return)
                    }
                    Token::Keyword(Keyword::Const | Keyword::Let | Keyword::Var) => self
                        .parse_variable_declaration()
                        .map(Statement::VariableDeclaration),
                    _ => self.parse_expression().map(Statement::Expression),
                };
                if stmt.is_some() {
                    self.expect_semicolon();
                }
                stmt
            }
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> Option<Expression> {
        let mut expression = self.parse_primary_expression()?;
        while let Some(&Token::Punctuator(punctuator)) = self.tokens.peek() {
            if let Some(op_kind) = Operator::try_parse(punctuator, Position::Postfix) {
                if op_kind.precedence() > min_precedence {
                    self.tokens.advance();
                    expression = self
                        .parse_secondary_expression(expression, op_kind)
                        .expect("Expected secondary expression but was <end>");
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(expression)
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        Some(match self.tokens.consume()? {
            Token::Identifier(identifier) => {
                let var_name = self.constants.allocate_if_absent(identifier);
                Expression::VariableAccess(VariableAccessExpression { var_name })
            }
            Token::Punctuator(Punctuator::OpenBrace) => Expression::Literal(LiteralExpression {
                value: if self
                    .tokens
                    .consume_eq(&Token::Punctuator(Punctuator::CloseBrace))
                    .is_some()
                {
                    ast::Literal::Object
                } else {
                    todo!(
                        "Parser::parse_primary_expression: Only empty object literals are \
                         supported"
                    )
                },
            }),
            Token::Punctuator(punc) => {
                if let Some(op_kind) = UnaryOperator::try_parse(punc, Position::Prefix) {
                    let operand = self
                        .parse_expression_impl(op_kind.precedence())
                        .expect("Expected expression but was <end>");
                    Expression::Unary(UnaryExpression {
                        kind: op_kind,
                        operand: Box::new(operand),
                    })
                } else if GroupingOp::try_parse(punc, Position::Prefix).is_some() {
                    let inner = self
                        .parse_expression()
                        .expect("Expected expression but was <end>");
                    self.tokens
                        .consume_exact(&Token::Punctuator(Punctuator::CloseParen));
                    Expression::Grouping(GroupingExpression {
                        inner: Box::new(inner),
                    })
                } else {
                    todo!(
                        "Parser::parse_primary_expression: token={}",
                        Token::Punctuator(punc)
                    )
                }
            }
            Token::Literal(literal) => Expression::Literal(LiteralExpression {
                value: match literal {
                    lexer::Literal::Boolean(value) => ast::Literal::Boolean(value),
                    lexer::Literal::Numeric(value) => ast::Literal::Numeric(value),
                    lexer::Literal::String(value) => ast::Literal::String(value),
                    lexer::Literal::Null => ast::Literal::Null,
                    lexer::Literal::Undefined => ast::Literal::Undefined,
                },
            }),
            Token::Keyword(Keyword::Function) => {
                let param_names = self.parse_fn_parameters();
                let body = self.parse_block();
                Expression::Literal(LiteralExpression {
                    value: ast::Literal::AnonFunction { param_names, body },
                })
            }
            token => todo!("Parser::parse_primary_expression: token={}", token),
        })
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        op_kind: Operator,
    ) -> Option<Expression> {
        Some(match op_kind {
            Operator::Assignment(kind) => Expression::Assignment(AssignmentExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(
                    self.parse_expression_impl(op_kind.precedence())
                        .expect("Expected right-hand-side of assignment expression but was <end>"),
                ),
            }),
            Operator::Binary(kind) => Expression::Binary(BinaryExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(
                    self.parse_expression_impl(op_kind.precedence())
                        .expect("Expected right-hand-side of binary expression but was <end>"),
                ),
            }),
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                kind,
                operand: Box::new(lhs),
            }),
            Operator::Ternary => {
                let condition = lhs;
                let lhs = self.parse_expression_impl(op_kind.precedence()).expect(
                    "Expected left-hand-side expression of ternary expression but was <end>",
                );
                match self.tokens.consume() {
                    Some(Token::Punctuator(Punctuator::Colon)) => {}
                    Some(token) => panic!(
                        "Expected colon between ternary expressions but was {}",
                        token
                    ),
                    None => panic!("Expected colon between ternary expressions but was <end>"),
                }
                let rhs = self.parse_expression_impl(op_kind.precedence()).expect(
                    "Expected right-hand-side expression of ternary expression but was <end>",
                );
                Expression::Ternary(TernaryExpression {
                    condition: Box::new(condition),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            Operator::Grouping => Expression::Grouping(GroupingExpression {
                inner: Box::new(lhs),
            }),
            Operator::FunctionCall => Expression::FunctionCall(FunctionCallExpression {
                function: Box::new(lhs),
                arguments: self.parse_fn_arguments(false),
            }),
            Operator::PropertyAccess => {
                let rhs = self
                    .parse_expression_impl(op_kind.precedence())
                    .expect("Expected right-hand-side of property access expression but was <end>");
                Expression::PropertyAccess(PropertyAccessExpression {
                    base: Box::new(lhs),
                    property_name: match rhs {
                        Expression::VariableAccess(VariableAccessExpression { var_name }) => {
                            var_name
                        }
                        rhs_expr => panic!("Expected property name but was {:#?}", rhs_expr),
                    },
                })
            }
        })
    }

    fn parse_function_declaration(&mut self) -> Option<FunctionDeclaration> {
        self.tokens
            .consume_exact(&Token::Keyword(Keyword::Function));
        if let Some(Token::Identifier(fn_name)) = self.tokens.consume() {
            let fn_name = self.constants.allocate_if_absent(fn_name);
            let param_names = self.parse_fn_parameters();
            let body = self.parse_block();
            Some(FunctionDeclaration {
                fn_name,
                param_names,
                body,
            })
        } else {
            panic!("Expected function name")
        }
    }

    fn parse_variable_declaration(&mut self) -> Option<VariableDeclaration> {
        let kind = match self.tokens.consume() {
            Some(Token::Keyword(Keyword::Const)) => VariableDeclarationKind::Const,
            Some(Token::Keyword(Keyword::Let)) => VariableDeclarationKind::Let,
            Some(Token::Keyword(Keyword::Var)) => VariableDeclarationKind::Var,
            Some(token) => panic!("Expected variable declaration but was {}", token),
            None => panic!("Expected variable declaration but was <end>"),
        };
        let mut entries = Vec::default();
        loop {
            let entry = self.parse_variable_declaration_entry();
            entries.push(entry);

            match self.tokens.peek() {
                Some(Token::Punctuator(Punctuator::Comma)) => {
                    self.tokens.consume();
                }
                Some(Token::Punctuator(Punctuator::Semicolon)) => break,
                Some(token) => panic!("Expected comma or semicolon but was {}", token),
                None => panic!("Expected comma or semicolon but was <end>"),
            }
        }
        Some(VariableDeclaration { kind, entries })
    }

    fn parse_variable_declaration_entry(&mut self) -> VariableDeclarationEntry {
        let var_name = match self.tokens.consume() {
            Some(Token::Identifier(var_name)) => self.constants.allocate_if_absent(var_name),
            Some(token) => unreachable!("Expected variable name but was {}", token),
            None => unreachable!("Expected variable name but was <end>"),
        };
        let initialiser = if let Some(Token::Punctuator(Punctuator::Equal)) = self.tokens.peek() {
            self.tokens.consume();
            let expr = self
                .parse_expression()
                .expect("Expected expression but was <end>");
            Some(expr)
        } else {
            None
        };
        VariableDeclarationEntry {
            var_name,
            initialiser,
        }
    }

    fn parse_assertion(&mut self) -> Option<Assertion> {
        self.tokens.consume_exact(&Token::Keyword(Keyword::Assert));
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        Some(Assertion { condition })
    }

    fn parse_print_statement(&mut self) -> Option<PrintStatement> {
        let new_line = match self.tokens.consume() {
            Some(Token::Keyword(Keyword::Print)) => false,
            Some(Token::Keyword(Keyword::PrintLn)) => true,
            Some(token) => unreachable!(
                "Expected `{}` or `{}` but was {}",
                Keyword::Print,
                Keyword::PrintLn,
                token
            ),
            None => unreachable!(
                "Expected `{}` or `{}` but was <end>",
                Keyword::Print,
                Keyword::PrintLn
            ),
        };
        let argument = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        Some(PrintStatement { argument, new_line })
    }

    fn parse_if_statement(&mut self) -> Option<IfStatement> {
        self.tokens.consume_exact(&Token::Keyword(Keyword::If));
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::CloseParen));
        let success_block = self.parse_block();
        let else_block = if self
            .tokens
            .consume_eq(&Token::Keyword(Keyword::Else))
            .is_some()
        {
            Some(self.parse_block())
        } else {
            None
        };
        Some(IfStatement {
            condition,
            success_block,
            else_block,
        })
    }

    fn parse_for_loop(&mut self) -> Option<ForLoop> {
        self.tokens.consume_exact(&Token::Keyword(Keyword::For));
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));

        let initialiser = match self.tokens.peek() {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            Some(_) => Some(
                self.parse_variable_declaration()
                    .expect("Expected variable declaration but was <end>"),
            ),
            None => panic!("Expected variable declaration or <end>"),
        };
        self.expect_semicolon();

        let condition = match self.tokens.peek() {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            Some(_) => Some(
                self.parse_expression()
                    .expect("Expected expression but was <end>"),
            ),
            None => panic!("Expected expression or semicolon but was <end>"),
        };
        self.expect_semicolon();

        let incrementor = match self.tokens.peek() {
            Some(Token::Punctuator(Punctuator::CloseParen)) => None,
            Some(_) => Some(
                self.parse_expression()
                    .expect("Expected expression but was <end>"),
            ),
            None => panic!("Expected expression or close paren but was <end>"),
        };
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::CloseParen));

        let block = self.parse_block();
        Some(ForLoop {
            initialiser,
            condition,
            incrementor,
            block,
        })
    }

    fn parse_while_loop(&mut self) -> Option<WhileLoop> {
        self.tokens.consume_exact(&Token::Keyword(Keyword::While));
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
        let condition = self
            .parse_expression()
            .expect("Expected expression but was <end>");
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::CloseParen));
        let block = self.parse_block();
        Some(WhileLoop { condition, block })
    }

    fn parse_break_statement(&mut self) -> Option<BreakStatement> {
        self.tokens.consume_exact(&Token::Keyword(Keyword::Break));
        Some(BreakStatement {})
    }

    fn parse_continue_statement(&mut self) -> Option<ContinueStatement> {
        self.tokens
            .consume_exact(&Token::Keyword(Keyword::Continue));
        Some(ContinueStatement {})
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.tokens.consume_exact(&Token::Keyword(Keyword::Return));
        let expr = match self.tokens.peek() {
            Some(Token::Punctuator(Punctuator::Semicolon)) => None,
            Some(_) => Some(
                self.parse_expression()
                    .expect("Expected expression but was <end>"),
            ),
            None => panic!("Expected expression or semicolon but was <end>"),
        };
        Some(ReturnStatement { expr })
    }

    fn parse_fn_parameters(&mut self) -> Vec<ConstantId> {
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
        if self
            .tokens
            .consume_eq(&Token::Punctuator(Punctuator::CloseParen))
            .is_some()
        {
            return Vec::with_capacity(0);
        }

        let mut params = Vec::new();
        loop {
            if let Some(Token::Identifier(param)) = self.tokens.consume() {
                let param_constant_id = self.constants.allocate_if_absent(param);
                params.push(param_constant_id);
            } else {
                panic!("Expected identifier (parameter name)");
            }
            match self.tokens.consume() {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break params,
                Some(token) => panic!("Expected comma or closing parenthesis but was {:?}", token),
                None => panic!("Expected comma or closing parenthesis but was <end>"),
            }
        }
    }

    fn parse_fn_arguments(&mut self, consume_open_paren: bool) -> Vec<Expression> {
        if consume_open_paren {
            self.tokens
                .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
        }
        if self
            .tokens
            .consume_eq(&Token::Punctuator(Punctuator::CloseParen))
            .is_some()
        {
            return Vec::with_capacity(0);
        }

        let mut args = Vec::new();
        loop {
            if let Some(arg) = self.parse_expression() {
                args.push(arg);
            } else {
                panic!("Expected expression but was <end>");
            }
            match self.tokens.consume() {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break args,
                Some(token) => panic!("Expected comma or closing parenthesis but was {:?}", token),
                None => panic!("Expected comma or closing parenthesis but was <end>"),
            }
        }
    }

    fn expect_semicolon(&mut self) {
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::Semicolon));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Position {
    Prefix,
    Postfix,
}

trait TryParse {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self>
    where
        Self: Sized;
}

impl TryParse for Operator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(if let Some(op) = AssignmentOperator::try_parse(punc, pos) {
            Self::Assignment(op)
        } else if let Some(op) = BinaryOperator::try_parse(punc, pos) {
            Self::Binary(op)
        } else if let Some(op) = UnaryOperator::try_parse(punc, pos) {
            Self::Unary(op)
        } else if TernaryOp::try_parse(punc, pos).is_some() {
            Self::Ternary
        } else if GroupingOp::try_parse(punc, pos).is_some() {
            Self::Grouping
        } else if FunctionCallOp::try_parse(punc, pos).is_some() {
            Self::FunctionCall
        } else if PropertyAccessOp::try_parse(punc, pos).is_some() {
            Self::PropertyAccess
        } else {
            return None;
        })
    }
}

impl TryParse for AssignmentOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::Postfix {
            return None;
        }
        Some(match punc {
            Punctuator::Equal => Self::Assign,

            Punctuator::PlusEqual => Self::AddAssign,
            Punctuator::SlashEqual => Self::DivAssign,
            Punctuator::PercentEqual => Self::ModAssign,
            Punctuator::AsteriskEqual => Self::MulAssign,
            Punctuator::DoubleAsteriskEqual => Self::PowAssign,
            Punctuator::MinusEqual => Self::SubAssign,

            Punctuator::DoubleLessThanEqual => Self::ShiftLeftAssign,
            Punctuator::DoubleMoreThanEqual => Self::ShiftRightAssign,
            Punctuator::TripleMoreThanEqual => Self::ShiftRightUnsignedAssign,

            Punctuator::AmpersandEqual => Self::BitwiseAndAssign,
            Punctuator::PipeEqual => Self::BitwiseOrAssign,
            Punctuator::CaretEqual => Self::BitwiseXOrAssign,

            _ => return None,
        })
    }
}

impl TryParse for BinaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::Postfix {
            return None;
        }
        Some(match punc {
            Punctuator::Plus => Self::Add,
            Punctuator::Slash => Self::Div,
            Punctuator::Percent => Self::Mod,
            Punctuator::Asterisk => Self::Mul,
            Punctuator::DoubleAsterisk => Self::Pow,
            Punctuator::Minus => Self::Sub,

            Punctuator::DoubleEqual => Self::Equal,
            Punctuator::BangEqual => Self::NotEqual,
            Punctuator::TripleEqual => Self::Identical,
            Punctuator::BangDoubleEqual => Self::NotIdentical,

            Punctuator::LessThan => Self::LessThan,
            Punctuator::LessThanEqual => Self::LessThanOrEqual,
            Punctuator::MoreThan => Self::MoreThan,
            Punctuator::MoreThanEqual => Self::MoreThanOrEqual,

            Punctuator::DoubleLessThan => Self::ShiftLeft,
            Punctuator::DoubleMoreThan => Self::ShiftRight,
            Punctuator::TripleMoreThan => Self::ShiftRightUnsigned,

            Punctuator::Ampersand => Self::BitwiseAnd,
            Punctuator::Pipe => Self::BitwiseOr,
            Punctuator::Caret => Self::BitwiseXOr,

            Punctuator::DoubleAmpersand => Self::LogicalAnd,
            Punctuator::DoublePipe => Self::LogicalOr,

            _ => return None,
        })
    }
}

impl TryParse for UnaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Punctuator::DoubleMinus, Position::Prefix) => Self::DecrementPrefix,
            (Punctuator::DoubleMinus, Position::Postfix) => Self::DecrementPostfix,
            (Punctuator::DoublePlus, Position::Prefix) => Self::IncrementPrefix,
            (Punctuator::DoublePlus, Position::Postfix) => Self::IncrementPostfix,

            (Punctuator::Tilde, Position::Prefix) => Self::BitwiseNot,
            (Punctuator::Bang, Position::Prefix) => Self::LogicalNot,
            (Punctuator::Minus, Position::Prefix) => Self::NumericNegate,
            (Punctuator::Plus, Position::Prefix) => Self::NumericPlus,

            (_, _) => return None,
        })
    }
}

impl TryParse for TernaryOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::Question, Position::Postfix)).then_some(Self)
    }
}

impl TryParse for GroupingOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::OpenParen, Position::Prefix)).then_some(Self)
    }
}

impl TryParse for FunctionCallOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::OpenParen, Position::Postfix)).then_some(Self)
    }
}

impl TryParse for PropertyAccessOp {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::Dot, Position::Postfix)).then_some(Self)
    }
}
