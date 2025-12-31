
#[derive(Debug, Copy, Clone)]
pub enum Value {
    Temp(u32),
    Var(u32),
    Ptr(u32),
    Const(i32),
    Reg(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub u32);

#[derive(Debug, Clone)]
pub enum Instr {
    Immediate { dst: Value, value: Value },
    Move { dst: Value, src: Value },
    Load { dst: Value, src: Value },
    Store { dst: Value, src: Value },

    Add { dst: Value, lhs: Value, rhs: Value },
    Sub { dst: Value, lhs: Value, rhs: Value },
    AddImmediate { dst: Value, lhs: Value, imm: i32 },
    Mul { dst: Value, lhs: Value, rhs: Value },

    CmpGt { dst: Value, lhs: Value, rhs: Value },

    Jump(Label),
    JumpIfFalse { cond: Value, target: Label },

    Label(Label),
}