use crate::id::TypeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StackSlot(usize);

impl StackSlot {
    pub const fn to_usize(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StackAllocation {
    pub ty: TypeId,
}

#[derive(Clone, Debug, Default)]
pub struct Stack {
    slots: Vec<StackAllocation>,
}

impl Stack {
    pub const fn new() -> Self {
        Self { slots: Vec::new() }
    }

    pub fn allocate(&mut self, allocation: StackAllocation) -> StackSlot {
        let slot = StackSlot(self.slots.len());
        self.slots.push(allocation);
        slot
    }

    pub fn get(&self, slot: StackSlot) -> Option<&StackAllocation> {
        self.slots.get(slot.0)
    }
}
