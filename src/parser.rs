use crate::{ast::{Expr, ExprNode, Stmt, StmtNode}, errors::CompileError, token::{Keyword, Token, TokenType, TokenValue}};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

const EOF_TOKEN: Token = Token {
    token_type: TokenType::EOF,
    value: TokenValue::None,
    position: 0,
};

pub type ParseResult<T> = Result<T, CompileError>;

impl Parser {
    // Logical
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&EOF_TOKEN)
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        self.pos += 1;
        token
    }

    fn check(&self, token_type: &TokenType) -> bool {
        &self.current().token_type == token_type
    }

    fn matches(&mut self, kinds: &[TokenType]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expect(&mut self, kind: &TokenType, msg: &str) -> ParseResult<Token> {
        if self.current().token_type == *kind {
            Ok(self.advance())
        } else {
            Err(CompileError { message: msg.to_string(), position: self.current().position })
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current().token_type, TokenType::EOF)
    }

    // Main parsing logic
    pub fn parse_program(&mut self) -> ParseResult<Vec<StmtNode>> {
        let mut stmts = Vec::new();

        while !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }

        Ok(stmts)
    }

    // Syntax
    fn parse_statement(&mut self) -> ParseResult<StmtNode> {
        match self.current() {
            Token { token_type: TokenType::KEYWORD, value: TokenValue::Keyword(Keyword::IF), .. } => self.parse_if(),
            Token { token_type: TokenType::KEYWORD, value: TokenValue::Keyword(Keyword::WHILE), .. } => self.parse_while(),
            Token { token_type: TokenType::KEYWORD, value: TokenValue::Keyword(Keyword::LET), .. } => self.parse_decleration(),
            Token { token_type: TokenType::IDENTIFIER, .. } => self.parse_assignment(),
            Token { token_type, ..} => Err(CompileError {
                message: format!("unexpected token {:?}", token_type),
                position: self.current().position,
            }),
        }
    }

    fn parse_if(&mut self) -> ParseResult<StmtNode> {
        let keyword = self.advance();

        let condition = self.parse_expression()?;
        let block = self.parse_block()?;

        Ok(StmtNode {
            node: Stmt::If { condition, body: block },
            position: keyword.position,
        })
    }

    fn parse_while(&mut self) -> ParseResult<StmtNode> {
        let keyword = self.advance();

        let condition = self.parse_expression()?;
        let block = self.parse_block()?;

        Ok(StmtNode {
            node: Stmt::While { condition, body: block },
            position: keyword.position,
        })
    }

    fn parse_block(&mut self) -> ParseResult<Vec<StmtNode>> {
        self.expect(&TokenType::LBRACE, "expected '{'")?;

        let mut stmts = Vec::new();
        while !self.check(&TokenType::RBRACE) {
            stmts.push(self.parse_statement()?);
        }

        self.expect(&TokenType::RBRACE, "expected '}'")?;
        Ok(stmts)
    }

    fn parse_decleration(&mut self) -> ParseResult<StmtNode> {
        let keyword = self.advance();

        let target = match self.expect(&TokenType::IDENTIFIER, "expected identifier")?.value {
            TokenValue::Identifier(n) => ExprNode { node: Expr::Variable(n), position: self.current().position },
            _ => unreachable!()
        };

        self.expect(&TokenType::ASSIGN, "expected '='")?;

        let value = self.parse_expression()?;

        self.expect(&TokenType::SEMICOLON, "expected ';'")?;

        Ok(StmtNode {
            node: Stmt::Declare { target, value },
            position: keyword.position,
        })
    }

    fn parse_assignment(&mut self) -> ParseResult<StmtNode> {
        let target = match self.expect(&TokenType::IDENTIFIER, "expected identifier")?.value {
            TokenValue::Identifier(n) => ExprNode { node: Expr::Variable(n), position: self.current().position },
            _ => unreachable!()
        };

        self.expect(&TokenType::ASSIGN, "expected '='")?;

        let value = self.parse_expression()?;

        self.expect(&TokenType::SEMICOLON, "expected ';'")?;

        Ok(StmtNode {
            position: target.position,
            node: Stmt::Assign { target, value },
        })
    }

    fn parse_expression(&mut self) -> ParseResult<ExprNode> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> ParseResult<ExprNode> {
        let mut expr = self.parse_term()?;

        while self.matches(&[
            TokenType::GT,
            TokenType::GTE,
            TokenType::LT,
            TokenType::LTE,
            TokenType::EQ,
            TokenType::NEQ,
        ]) {
            let op = self.tokens[self.pos - 1].clone();
            let right = self.parse_term()?;

            expr = ExprNode {
                position: expr.position,
                node: Expr::Binary {
                    left: Box::new(expr),
                    operator: op.token_type,
                    right: Box::new(right),
                },
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> ParseResult<ExprNode> {
        let mut expr = self.parse_factor()?;

        while matches!(self.current().token_type, TokenType::PLUS | TokenType::MINUS) {
            let op = self.advance();
            let right = self.parse_factor()?;

            expr = ExprNode {
                position: expr.position,
                node: Expr::Binary {
                    left: Box::new(expr),
                    operator: op.token_type,
                    right: Box::new(right),
                },
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> ParseResult<ExprNode> {
        let mut expr = self.parse_unary()?;

        while self.check(&TokenType::ASTERISK) || self.check(&TokenType::SLASH) {
            let op = self.advance();
            let right = self.parse_unary()?;

            expr = ExprNode {
                position: expr.position,
                node: Expr::Binary {
                    left: Box::new(expr),
                    operator: op.token_type,
                    right: Box::new(right),
                },
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> ParseResult<ExprNode> {
        if self.check(&TokenType::MINUS) {
            let op = self.advance();
            let right = self.parse_unary()?;

            return Ok(ExprNode {
                node: Expr::Unary {
                    operator: op.token_type,
                    operand: Box::new(right),
                },
                position: op.position,
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> ParseResult<ExprNode> {
        match self.advance() {
            Token { token_type: TokenType::NUMBER, value: TokenValue::Number(n), .. } => Ok(ExprNode { node: Expr::Number(n), position: self.current().position }),
            Token { token_type: TokenType::IDENTIFIER, value: TokenValue::Identifier(name), .. } => Ok(ExprNode { node: Expr::Variable(name), position: self.current().position }),

            Token { token_type: TokenType::LPAREN, .. } => {
                let expr = self.parse_expression()?;
                self.expect(&TokenType::RPAREN, "expected ')'")?;
                Ok(expr)
            }

            token => Err(CompileError {
                message: format!("expected expression, found {:?}", token.token_type),
                position: token.position,
            })
        }
    }
}