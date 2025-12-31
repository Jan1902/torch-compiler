use crate::instructions::{Instr, Value};

const NUM_REGISTERS: usize = 7;

pub struct Allocator {
    instrs: Vec<Instr>,
    regs: [Option<Value>; NUM_REGISTERS],
    ram: Vec<Value>,
    dirty: [bool; NUM_REGISTERS],
}

impl Allocator {
    pub fn new() -> Self {
        Self {
            instrs: Vec::new(),
            regs: [None; NUM_REGISTERS],
            ram: Vec::new(),
            dirty: [false; NUM_REGISTERS],
        }
    }

    pub fn allocate(&mut self, instrs: &Vec<Instr>) -> Vec<Instr> {
        for instr in instrs.iter() {
            match instr {
                Instr::Move { dst, src } => {
                    let src_reg = self.get_or_load(src, &[]);
                    let dst_reg = self.allocate_register(dst, &[Self::get_id_of(&src_reg) as u8]);
                    self.instrs.push(Instr::Move {
                        dst: dst_reg,
                        src: src_reg,
                    });
                    self.dirty[Self::get_id_of(&dst_reg) as usize] = true;
                }
                Instr::Immediate { dst, value } => {
                    let dst_reg = self.allocate_register(dst, &[]);
                    self.instrs.push(Instr::Immediate {
                        dst: dst_reg,
                        value: value.clone(),
                    });
                    self.dirty[Self::get_id_of(&dst_reg) as usize] = true;
                }
                Instr::Add { dst, lhs, rhs } => {
                    let lhs_reg = self.get_or_load(lhs, &[]);
                    let rhs_reg = self.get_or_load(rhs, &[Self::get_id_of(&lhs_reg) as u8]);
                    let dst_reg = self.allocate_register(
                        dst,
                        &[
                            Self::get_id_of(&lhs_reg) as u8,
                            Self::get_id_of(&rhs_reg) as u8,
                        ],
                    );
                    self.instrs.push(Instr::Add {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        rhs: rhs_reg,
                    });
                    self.dirty[Self::get_id_of(&dst_reg) as usize] = true;
                }
                Instr::AddImmediate { dst, lhs, imm } => {
                    let lhs_reg = self.get_or_load(lhs, &[]);
                    let dst_reg = self.allocate_register(dst, &[Self::get_id_of(&lhs_reg) as u8]);
                    self.instrs.push(Instr::AddImmediate {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        imm: *imm,
                    });
                    self.dirty[Self::get_id_of(&dst_reg) as usize] = true;
                }
                Instr::Sub { dst, lhs, rhs } => {
                    let lhs_reg = self.get_or_load(lhs, &[]);
                    let rhs_reg = self.get_or_load(rhs, &[Self::get_id_of(&lhs_reg) as u8]);
                    let dst_reg = self.allocate_register(
                        dst,
                        &[
                            Self::get_id_of(&lhs_reg) as u8,
                            Self::get_id_of(&rhs_reg) as u8,
                        ],
                    );
                    self.instrs.push(Instr::Sub {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        rhs: rhs_reg,
                    });
                    self.dirty[Self::get_id_of(&dst_reg) as usize] = true;
                }
                _ => {
                    self.instrs.push(instr.clone());
                }
            }
        }

        self.instrs.clone()
    }

    fn allocate_register(&mut self, value: &Value, locked_regs: &[u8]) -> Value {
        for reg in 0..NUM_REGISTERS {
            if self.regs[reg].is_none() && !locked_regs.contains(&(reg as u8)) {
                self.regs[reg] = Some(value.clone());
                return Value::Reg(reg as u8);
            }
        }

        // Simple spill strategy: spill the first register - for demonstration purposes
        let spilled_value = self.regs[self.pick_spill_register(locked_regs)]
            .take()
            .unwrap();
        self.ram.push(spilled_value.clone());

        self.instrs.push(Instr::Store {
            dst: spilled_value,
            src: Value::Reg(0),
        });
        self.regs[0] = Some(value.clone());
        Value::Reg(0)
    }

    fn get_or_load(&mut self, value: &Value, locked_regs: &[u8]) -> Value {
        for reg in 0..NUM_REGISTERS {
            if let Some(v) = &self.regs[reg] {
                if Self::get_id_of(v) == Self::get_id_of(value) {
                    return Value::Reg(reg as u8);
                }
            }
        }

        let reg = self.allocate_register(value, locked_regs);
        self.instrs.push(Instr::Load {
            dst: reg.clone(),
            src: value.clone(),
        });

        reg
    }

    fn pick_spill_register(&self, locked_regs: &[u8]) -> usize {
        for reg in 0..NUM_REGISTERS {
            if !locked_regs.contains(&(reg as u8)) && !self.dirty[reg] {
                return reg;
            }
        }

        for i in 0..7 {
            if !locked_regs.contains(&(i as u8)) {
                return i;
            }
        }

        panic!("No register available to spill");
    }

    fn get_id_of(value: &Value) -> u32 {
        match value {
            Value::Temp(id) | Value::Var(id) | Value::Ptr(id) => *id,
            Value::Reg(id) => *id as u32,
            _ => panic!("Value does not have an ID"),
        }
    }
}
