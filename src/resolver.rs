use crate::{ast::{Expr, ExprNode, Stmt, StmtNode}, errors::CompileError, symbols::SymbolTable};

pub struct Resolver<'a> {
    table: &'a mut SymbolTable,
}

impl<'a> Resolver<'a> {
    pub fn new(table: &'a mut SymbolTable) -> Self {
        Resolver {
            table,
        }
    }

    fn resolve(&mut self, name: &str, pos: usize, source_id: usize) -> Result<(), CompileError> {
        self.table.resolve(name).map_err(|msg| CompileError {
            position: pos,
            source_id,
            message: msg,
        })?;

        Ok(())
    }

    fn define(&mut self, name: &str, pos: usize, source_id: usize) -> Result<(), CompileError> {
        self.table.define(name, pos, source_id).map_err(|msg| CompileError {
            position: pos,
            source_id,
            message: msg,
        })?;

        Ok(())
    }

    pub fn resolve_program(mut self, stmts: &[StmtNode]) -> Result<(), CompileError> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }

        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &StmtNode) -> Result<(), CompileError> {
        match &stmt.node {
            Stmt::Declare { target, value } => {
                self.resolve_expr(value)?;

                if let ExprNode { node: Expr::Variable(name), position: _, source_id: _ } = target {
                    self.define(name, target.position, target.source_id)?;
                }
            }

            Stmt::Assign { target, value } => {
                self.resolve_expr(value)?;

                self.resolve_expr(target)?;
            }

            Stmt::If { condition, body } |
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;

                self.table.begin_scope();

                for stmt in body {
                    self.resolve_stmt(stmt)?;
                }

                self.table.end_scope();
            }
        }

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &ExprNode) -> Result<(), CompileError> {
        match &expr.node {
            Expr::Variable(name) => {
                self.resolve(name, expr.position, expr.source_id)?;
            }

            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }

            Expr::Unary { operand, .. } => {
                self.resolve_expr(operand)?;
            }

            Expr::Number(_) => {}
        }

        Ok(())
    }
}