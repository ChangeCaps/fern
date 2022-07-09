use crate::{
    id::FunctionId,
    ir::{Register, StackSlot},
};

use super::Type;

#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub ty: Type,
    pub kind: ValueKind,
}

impl Value {
    pub fn new(ty: Type, kind: impl Into<ValueKind>) -> Self {
        Self {
            ty,
            kind: kind.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ValueKind {
    Register(Register),
    Stack(StackSlot),
    Function(FunctionId),
}

impl From<Register> for ValueKind {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl From<StackSlot> for ValueKind {
    fn from(slot: StackSlot) -> Self {
        Self::Stack(slot)
    }
}

impl From<FunctionId> for ValueKind {
    fn from(id: FunctionId) -> Self {
        Self::Function(id)
    }
}
