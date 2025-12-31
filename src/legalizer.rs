use crate::instructions::{Instr, Value};

pub struct Legalizer<'a> {
    instrs: &'a Vec<Instr>,
    next_temp: u32,
}

impl<'a> Legalizer<'a> {
    pub fn new(instrs: &'a Vec<Instr>) -> Self {
        Self { instrs, next_temp: instrs.iter().filter_map(|instr| {
            match instr {
                Instr::Immediate { dst, .. } | Instr::Move { dst, .. } | Instr::Load { dst, .. } | Instr::Store { dst, .. } |
                Instr::Add { dst, .. } | Instr::Sub { dst, .. } | Instr::AddImmediate { dst, .. } | Instr::Mul { dst, .. } |
                Instr::CmpGt { dst, .. } => {
                    if let Value::Temp(id) = dst {
                        Some(*id)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }).max().unwrap_or(0) + 1 }
    }

    fn get_next_temp(&mut self) -> Value {
        let temp = Value::Temp(self.next_temp);
        self.next_temp += 1;
        temp
    }

    pub fn legalize(&mut self) -> Vec<Instr> {
        let mut legalized_instrs = Vec::new();

        for instr in self.instrs.iter() {
            match instr {
                Instr::Move { dst, src } => match (dst, src) {
                    (Value::Var(_), Value::Const(_)) => {
                        legalized_instrs.push(Instr::Immediate {
                            dst: dst.clone(),
                            value: src.clone(),
                        });
                    }
                    _ => {
                        legalized_instrs.push(instr.clone());
                    }
                },
                Instr::Add { dst, lhs, rhs } => match (lhs, rhs) {
                    (Value::Const(l), Value::Const(r)) => {
                        legalized_instrs.push(Instr::Immediate {
                            dst: dst.clone(),
                            value: Value::Const(l + r),
                        });
                    }
                    (val, Value::Const(c)) | (Value::Const(c), val) => {
                        legalized_instrs.push(Instr::AddImmediate {
                            dst: dst.clone(),
                            lhs: val.clone(),
                            imm: *c,
                        });
                    }
                    _ => {
                        legalized_instrs.push(instr.clone());
                    }
                },
                Instr::Sub { dst, lhs, rhs } => {
                    match (lhs, rhs) {
                        (Value::Const(l), Value::Const(r)) => {
                            legalized_instrs.push(Instr::Immediate {
                                dst: dst.clone(),
                                value: Value::Const(l - r),
                            });
                        }
                        (val, Value::Const(c)) | (Value::Const(c), val) => {
                            // legalized_instrs.push(Instr::AddImmediate {
                            //     dst: dst.clone(),
                            //     lhs: val.clone(),
                            //     imm: -(*c),
                            // });
                            let temp = self.get_next_temp();
                            legalized_instrs.push(Instr::Immediate { dst: temp, value: Value::Const(*c) });
                            legalized_instrs.push(Instr::Sub { dst: dst.clone(), lhs: val.clone(), rhs: temp });
                        }
                        _ => {
                            legalized_instrs.push(instr.clone());
                        }
                    }
                }
                _ => {
                    legalized_instrs.push(instr.clone());
                }
            }
        }

        legalized_instrs
    }
}
