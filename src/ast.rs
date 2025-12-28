use crate::token::TokenType;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Binary {
        left: Box<ExprNode>,
        operator: TokenType,
        right: Box<ExprNode>,
    },
    Unary {
        operator: TokenType,
        operand: Box<ExprNode>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Declare {
        target: ExprNode,
        value: ExprNode,
    },
    Assign {
        target: ExprNode,
        value: ExprNode,
    },
    If {
        condition: ExprNode,
        body: Vec<StmtNode>,
    },
    While {
        condition: ExprNode,
        body: Vec<StmtNode>,
    },
}

#[derive(Debug)]
pub struct AstNode<T> {
    pub node: T,
    pub position: usize,
}

pub type ExprNode = AstNode<Expr>;
pub type StmtNode = AstNode<Stmt>;