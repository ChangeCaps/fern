use crate::id::FunctionId;

use super::{Block, Immediate, MemoryType, Register, RegisterAllocator, StackSlot};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Opcode(u8);

macro_rules! instructions {
    ($($opcode:literal: $ident:ident { $($field:ident : $ty:ty),* $(,)? }),* $(,)?) => {
        #[derive(Clone, Debug)]
        pub enum Instruction {
            $($ident { $($field: $ty),* },)*
        }

        impl Instruction {
            pub const fn opcode(&self) -> Opcode {
                match self {
                    $(Self::$ident { .. } => Opcode($opcode),)*
                }
            }
        }
    };
}

instructions! {
    0: Noop {},
    1: IConst { dst: Register, imm: Immediate, ty: MemoryType },
    16: Call { dst: Register, function: FunctionId, arguments: Vec<Register> },
    18: Return { src: Register },
    32: Add { dst: Register, lhs: Register, rhs: Register },
    33: Sub { dst: Register, lhs: Register, rhs: Register },
    34: Mul { dst: Register, lhs: Register, rhs: Register },
    35: Div { dst: Register, lhs: Register, rhs: Register },
    48: FuncAddr { dst: Register, function: FunctionId, ty: MemoryType },
    64: StackLoad { dst: Register, slot: StackSlot, ty: MemoryType, offset: u32 },
    65: StackStore { src: Register, slot: StackSlot, ty: MemoryType, offset: u32 },
    66: StackAddr { dst: Register, slot: StackSlot, ty: MemoryType },
    72: Load { dst: Register, src: Register, ty: MemoryType, offset: u32 },
    73: Store { dst: Register, src: Register, ty: MemoryType, offset: u32 },
}

pub struct InstructionBuilder<'a> {
    pub(crate) register_allocator: &'a mut RegisterAllocator,
    pub(crate) block: &'a mut Block,
}

impl<'a> InstructionBuilder<'a> {
    fn push(&mut self, instruction: Instruction) {
        self.block.push(instruction);
    }

    fn allocate_register(&mut self) -> Register {
        self.register_allocator.allocate()
    }

    pub fn noop(&mut self) {
        self.push(Instruction::Noop {});
    }

    pub fn iconst(&mut self, imm: impl Into<Immediate>, ty: MemoryType) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::IConst {
            dst,
            imm: imm.into(),
            ty,
        });

        dst
    }

    pub fn call(&mut self, function: FunctionId, arguments: impl Into<Vec<Register>>) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::Call {
            dst,
            function,
            arguments: arguments.into(),
        });

        dst
    }

    pub fn ret(&mut self, src: Register) {
        self.push(Instruction::Return { src });
    }

    pub fn add(&mut self, lhs: Register, rhs: Register) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::Add { dst, lhs, rhs });

        dst
    }

    pub fn sub(&mut self, lhs: Register, rhs: Register) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::Sub { dst, lhs, rhs });

        dst
    }

    pub fn mul(&mut self, lhs: Register, rhs: Register) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::Mul { dst, lhs, rhs });

        dst
    }

    pub fn div(&mut self, lhs: Register, rhs: Register) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::Div { dst, lhs, rhs });

        dst
    }

    pub fn func_addr(&mut self, function: FunctionId, ty: MemoryType) -> Register {
        let dst = self.allocate_register();
        self.push(Instruction::FuncAddr { dst, function, ty });
        dst
    }

    pub fn stack_load(&mut self, slot: StackSlot, ty: MemoryType, offset: u32) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::StackLoad {
            dst,
            slot,
            ty,
            offset,
        });

        dst
    }

    pub fn stack_store(&mut self, src: Register, slot: StackSlot, ty: MemoryType, offset: u32) {
        self.push(Instruction::StackStore {
            src,
            slot,
            ty,
            offset,
        });
    }

    pub fn stack_addr(&mut self, ty: MemoryType, slot: StackSlot) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::StackAddr { dst, slot, ty });

        dst
    }

    pub fn load(&mut self, src: Register, ty: MemoryType, offset: u32) -> Register {
        let dst = self.allocate_register();

        self.push(Instruction::Load {
            dst,
            src,
            ty,
            offset,
        });

        dst
    }

    pub fn store(&mut self, dst: Register, src: Register, ty: MemoryType, offset: u32) {
        self.push(Instruction::Store {
            dst,
            src,
            ty,
            offset,
        });
    }
}
