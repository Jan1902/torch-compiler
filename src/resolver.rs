use std::collections::HashMap;

use crate::{ast::{Expr, ExprNode, Stmt, StmtNode}, errors::CompileError, symbols::{Scope, Symbol}};

pub struct Resolver {
    scopes: Vec<Scope>,
    errors: Vec<CompileError>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scopes: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Scope {
            symbols: HashMap::new(),
        });
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve(&mut self, name: &str, pos: usize) {
        for scope in self.scopes.iter().rev() {
            if scope.symbols.contains_key(name) {
                return;
            }
        }

        self.errors.push(CompileError {
            position: pos,
            message: format!("use of undeclared variable `{}`", name),
        });
    }

    fn define(&mut self, name: &str, pos: usize) {
        let scope = self.scopes.last_mut().unwrap();

        if scope.symbols.contains_key(name) {
            self.errors.push(CompileError {
                position: pos,
                message: format!("variable `{}` already declared in this scope", name),
            });
        } else {
            scope.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    address: pos,
                },
            );
        }
    }

    pub fn resolve_program(mut self, stmts: &[StmtNode]) -> Result<(), Vec<CompileError>> {
        self.begin_scope(); // global scope

        for stmt in stmts {
            self.resolve_stmt(stmt);
        }

        self.end_scope();

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }

    fn resolve_stmt(&mut self, stmt: &StmtNode) {
        match &stmt.node {
            Stmt::Declare { target, value } => {
                self.resolve_expr(value);

                if let ExprNode { node: Expr::Variable(name), position: _ } = target {
                    self.define(name, target.position);
                }
            }

            Stmt::Assign { target, value } => {
                self.resolve_expr(value);

                if let ExprNode { node: Expr::Variable(name), position: _ } = target {
                    self.resolve(name, target.position); // Might also be able to just call resolve_expr here
                }
            }

            Stmt::If { condition, body } |
            Stmt::While { condition, body } => {
                self.resolve_expr(condition);

                self.begin_scope();
                for stmt in body {
                    self.resolve_stmt(stmt);
                }
                self.end_scope();
            }
        }
    }

    fn resolve_expr(&mut self, expr: &ExprNode) {
        match &expr.node {
            Expr::Variable(name) => {
                self.resolve(name, expr.position);
            }

            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }

            Expr::Unary { operand, .. } => {
                self.resolve_expr(operand);
            }

            Expr::Number(_) => {}
        }
    }
}