use crate::{ast::{Expr, Stmt, StmtNode}, instructions::{Instr, Label, Value}, symbols::SymbolTable, token::TokenType};

pub struct IrBuilder<'a> {
    instrs: Vec<Instr>,
    next_temp: u32,
    next_label: u32,
    symbols: &'a SymbolTable,
}

impl<'a> IrBuilder<'a> {
    pub fn new(symbols: &'a SymbolTable) -> Self {
        IrBuilder {
            instrs: Vec::new(),
            next_temp: symbols.scopes.iter().map(|s| s.symbols.len()).sum::<usize>() as u32 + 1,
            next_label: 0,
            symbols,
        }
    }

    fn emit(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    fn new_temp(&mut self) -> Value {
        let temp = Value::Temp(self.next_temp);
        self.next_temp += 1;
        temp
    }

    fn new_label(&mut self) -> Label {
        let label = Label(self.next_label);
        self.next_label += 1;
        label
    }

    fn lower_expr(&mut self, expr: &Expr, target: Option<Value>) -> Value {
        match expr {
            Expr::Number(n) => {
                if let Some(t) = target {
                    self.emit(Instr::Move { dst: t, src: Value::Const(*n) });
                    t
                } else {
                    Value::Const(*n)
                }
            },
            Expr::Variable(name) => {
                let id = self.symbols.id_of(name);
                if let Some(t) = target {
                    self.emit(Instr::Move { dst: t, src: Value::Var(id) });
                    t
                } else {
                    Value::Var(id)
                }
            },
            Expr::Binary { left, right, operator } => {
                let lhs = self.lower_expr(&left.node, None);
                let rhs = self.lower_expr(&right.node, None);
                let dst = match target {
                    Some(t) => t,
                    None => self.new_temp(),
                };

                self.emit(match operator {
                    TokenType::PLUS => Instr::Add { dst, lhs, rhs },
                    TokenType::MINUS => Instr::Sub { dst, lhs, rhs },
                    TokenType::GT => Instr::CmpGt { dst, lhs, rhs },
                    _ => unimplemented!(),
                });

                dst
            },
            Expr::Unary { operand, operator } => {
                // Beispiel: Negation
                let val = self.lower_expr(&operand.node, None);
                let dst = match target {
                    Some(t) => t,
                    None => self.new_temp(),
                };

                match operator {
                    TokenType::MINUS => {
                        // dst = 0 - val
                        self.emit(Instr::Sub {
                            dst,
                            lhs: Value::Const(0),
                            rhs: val,
                        });
                    }
                    _ => (),
                }

                dst
            }
        }
    }

    fn lower_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign { target, value } => {
                if let Expr::Variable(name) = &target.node {
                    let var = self.symbols.id_of(&name);

                    self.lower_expr(&value.node, Some(Value::Var(var)));
                } else {
                    panic!("Invalid assignment target");
                }
            },
            Stmt::Declare { target, value } => {
                if let Expr::Variable(name) = &target.node {
                    let var = self.symbols.id_of(&name);

                    self.lower_expr(&value.node, Some(Value::Var(var)));
                } else {
                    panic!("Invalid declaration target");
                }
            },
            Stmt::While { condition, body } => {
                let start = self.new_label();
                let end = self.new_label();

                self.emit(Instr::Label(start));

                let cond = self.lower_expr(&condition.node, None);
                self.emit(Instr::JumpIfFalse {
                    cond,
                    target: end,
                });

                for s in body {
                    self.lower_stmt(&s.node);
                }

                self.emit(Instr::Jump(start));
                self.emit(Instr::Label(end));
            },
            _ => unimplemented!(),
        }
    }

    pub fn build(&mut self, stmts: &Vec<StmtNode>) -> &Vec<Instr> {
        for stmt in stmts {
            self.lower_stmt(&stmt.node);
        }
        &self.instrs
    }
}