use crate::{ast::{Expr, Stmt}, symbols::SymbolTable, token::TokenType};

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Temp(u32),   // temporäres Register
    Var(u32),    // Variable (Symbol-ID!)
    Const(i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub u32);

#[derive(Debug)]
pub enum Instr {
    Load { dst: Value, src: Value },
    Store { dst: Value, src: Value },

    Add { dst: Value, lhs: Value, rhs: Value },
    Sub { dst: Value, lhs: Value, rhs: Value },

    CmpGt { dst: Value, lhs: Value, rhs: Value },

    Jump(Label),
    JumpIfFalse { cond: Value, target: Label },

    Label(Label),
}