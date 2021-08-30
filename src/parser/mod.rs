use crate::ast::{self, *};
use crate::lexer::{self, *};
use crate::util::Stream;
use std::convert::TryFrom;
use std::iter::Iterator;

pub struct Parser {
    tokens: Stream<Token>,
    constants: Vec<(ConstantId, ConstantValue)>,
}

impl Parser {
    pub fn for_lexer(source: Lexer) -> Self {
        Self::new(source)
    }

    pub fn for_tokens(source: Vec<Token>) -> Self {
        Self::new(source.into_iter())
    }

    pub fn new(source: impl Iterator<Item = Token>) -> Self {
        let tokens = Stream::new(source.filter(Token::is_significant));
        Self {
            tokens,
            constants: Vec::default(),
        }
    }
}

impl Parser {
    pub fn execute(mut self) -> Program {
        let mut stmts = Vec::new();
        while let Some(stmt) = self.parse_statement() {
            stmts.push(stmt);
        }
        Program::new(Block::new(stmts), self.constants)
    }

    fn alloc_or_get_constant(&mut self, value: ConstantValue) -> ConstantId {
        if let Some((id, _value)) = self
            .constants
            .iter()
            .find(|(_id, existing_value)| value.as_str() == existing_value)
        {
            *id
        } else {
            let id = ConstantId::new(self.constants.len());
            self.constants.push((id, value));
            id
        }
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
                // Block not closed before end of input
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
                    Token::Keyword(Keyword::Const | Keyword::Let) => self
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
            if let Ok(op_kind) = Op::try_from(punctuator) {
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
                let identifier = self.alloc_or_get_constant(identifier);
                let var_expr = Expression::VariableAccess(VariableAccessExpression {
                    var_name: identifier,
                });
                if self.tokens.peek() == Some(&Token::Punctuator(Punctuator::OpenParen)) {
                    let arguments = self.parse_fn_arguments();
                    Expression::FunctionCall(FunctionCallExpression {
                        function: Box::new(var_expr),
                        arguments,
                    })
                } else {
                    var_expr
                }
            }
            Token::Punctuator(Punctuator::OpenParen) => {
                let inner = self.parse_expression().expect("Expected expression");
                self.tokens
                    .consume_exact(&Token::Punctuator(Punctuator::CloseParen));
                Expression::Grouping(GroupingExpression {
                    inner: Box::new(inner),
                })
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
            Token::Literal(literal) => Expression::Literal(LiteralExpression {
                value: match literal {
                    lexer::Literal::Boolean(value) => ast::Literal::Boolean(value),
                    lexer::Literal::Numeric(value) => ast::Literal::Numeric(value),
                    lexer::Literal::String(value) => ast::Literal::String(value),
                    lexer::Literal::Null => ast::Literal::Null,
                    lexer::Literal::Undefined => ast::Literal::Undefined,
                },
            }),
            token => todo!("Parser::parse_primary_expression: token={}", token),
        })
    }

    fn parse_secondary_expression(&mut self, lhs: Expression, op_kind: Op) -> Option<Expression> {
        let rhs = self
            .parse_expression_impl(op_kind.precedence())
            .expect("Expected RHS of binary expression but was <end>");

        Some(match op_kind {
            Op::Assignment(kind) => Expression::Assignment(AssignmentExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Op::Binary(kind) => Expression::Binary(BinaryExpression {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Op::PropertyAccess(PropertyAccessOp::Normal) => {
                Expression::PropertyAccess(PropertyAccessExpression {
                    base: Box::new(lhs),
                    property_name: match rhs {
                        Expression::VariableAccess(VariableAccessExpression { var_name }) => {
                            var_name
                        }
                        Expression::FunctionCall(expr) => {
                            todo!("Parser::parse_secondary_expression: {:?}", expr)
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
            let fn_name = self.alloc_or_get_constant(fn_name);
            let param_names = self
                .parse_fn_parameters()
                .into_iter()
                .map(|param_name| self.alloc_or_get_constant(param_name))
                .collect();
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
            token => panic!("Expected variable declaration but was {:?}", token),
        };

        match self.tokens.consume() {
            Some(Token::Identifier(var_name)) => {
                let var_name = self.alloc_or_get_constant(var_name);
                let initialiser = match self.tokens.peek() {
                    Some(Token::Punctuator(Punctuator::Equal)) => {
                        self.tokens
                            .consume_exact(&Token::Punctuator(Punctuator::Equal));
                        Some(
                            self.parse_expression()
                                .expect("Expected expression but was <end>"),
                        )
                    }
                    Some(Token::Punctuator(Punctuator::Semicolon)) => None,
                    Some(token) => panic!("Expected initialiser or semicolon but was {}", token),
                    None => panic!("Expected initialiser or semicolon but was <end>"),
                };
                Some(VariableDeclaration {
                    kind,
                    var_name,
                    initialiser,
                })
            }
            Some(token) => unreachable!("Expected variable name but was {}", token),
            None => unreachable!("Expected variable name but was <end>"),
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

    fn parse_fn_parameters(&mut self) -> Vec<IdentifierName> {
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
                params.push(param);
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

    fn parse_fn_arguments(&mut self) -> Vec<Expression> {
        self.tokens
            .consume_exact(&Token::Punctuator(Punctuator::OpenParen));
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Assignment(AssignmentOp),
    Binary(BinaryOp),
    PropertyAccess(PropertyAccessOp),
}

impl Operator for Op {
    fn associativity(&self) -> Associativity {
        match self {
            Self::Assignment(op) => op.associativity(),
            Self::Binary(op) => op.associativity(),
            Self::PropertyAccess(op) => op.associativity(),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Assignment(op) => op.precedence(),
            Self::Binary(op) => op.precedence(),
            Self::PropertyAccess(op) => op.precedence(),
        }
    }
}

impl TryFrom<Punctuator> for Op {
    type Error = ();

    fn try_from(punc: Punctuator) -> Result<Self, Self::Error> {
        Ok(match punc {
            Punctuator::Equal => Self::Assignment(AssignmentOp::Assign),
            Punctuator::PlusEqual => Self::Assignment(AssignmentOp::AddAssign),
            Punctuator::SlashEqual => Self::Assignment(AssignmentOp::DivAssign),
            Punctuator::PercentEqual => Self::Assignment(AssignmentOp::ModAssign),
            Punctuator::AsteriskEqual => Self::Assignment(AssignmentOp::MulAssign),
            Punctuator::DoubleAsteriskEqual => Self::Assignment(AssignmentOp::PowAssign),
            Punctuator::MinusEqual => Self::Assignment(AssignmentOp::SubAssign),
            Punctuator::DoubleLessThanEqual => Self::Assignment(AssignmentOp::ShiftLeftAssign),
            Punctuator::DoubleMoreThanEqual => Self::Assignment(AssignmentOp::ShiftRightAssign),
            Punctuator::TripleMoreThanEqual => {
                Self::Assignment(AssignmentOp::ShiftRightUnsignedAssign)
            }
            Punctuator::AmpersandEqual => Self::Assignment(AssignmentOp::BitwiseAndAssign),
            Punctuator::PipeEqual => Self::Assignment(AssignmentOp::BitwiseOrAssign),
            Punctuator::CaretEqual => Self::Assignment(AssignmentOp::BitwiseXOrAssign),

            Punctuator::Plus => Self::Binary(BinaryOp::Add),
            Punctuator::Slash => Self::Binary(BinaryOp::Div),
            Punctuator::Percent => Self::Binary(BinaryOp::Mod),
            Punctuator::Asterisk => Self::Binary(BinaryOp::Mul),
            Punctuator::DoubleAsterisk => Self::Binary(BinaryOp::Pow),
            Punctuator::Minus => Self::Binary(BinaryOp::Sub),
            Punctuator::DoubleEqual => Self::Binary(BinaryOp::Equal),
            Punctuator::BangEqual => Self::Binary(BinaryOp::NotEqual),
            Punctuator::TripleEqual => Self::Binary(BinaryOp::Identical),
            Punctuator::BangDoubleEqual => Self::Binary(BinaryOp::NotIdentical),
            Punctuator::LessThan => Self::Binary(BinaryOp::LessThan),
            Punctuator::LessThanEqual => Self::Binary(BinaryOp::LessThanOrEqual),
            Punctuator::MoreThan => Self::Binary(BinaryOp::MoreThan),
            Punctuator::MoreThanEqual => Self::Binary(BinaryOp::MoreThanOrEqual),
            Punctuator::DoubleLessThan => Self::Binary(BinaryOp::ShiftLeft),
            Punctuator::DoubleMoreThan => Self::Binary(BinaryOp::ShiftRight),
            Punctuator::TripleMoreThan => Self::Binary(BinaryOp::ShiftRightUnsigned),
            Punctuator::Ampersand => Self::Binary(BinaryOp::BitwiseAnd),
            Punctuator::Pipe => Self::Binary(BinaryOp::BitwiseOr),
            Punctuator::Caret => Self::Binary(BinaryOp::BitwiseXOr),
            Punctuator::DoubleAmpersand => Op::Binary(BinaryOp::LogicalAnd),
            Punctuator::DoublePipe => Self::Binary(BinaryOp::LogicalOr),

            Punctuator::Dot => Self::PropertyAccess(PropertyAccessOp::Normal),

            _ => return Err(()),
        })
    }
}
