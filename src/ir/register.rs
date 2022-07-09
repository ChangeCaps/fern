use std::collections::BTreeSet;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(u32);

impl Register {
    pub const fn to_u32(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct RegisterAllocator {
    free: BTreeSet<Register>,
    count: u32,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            free: BTreeSet::new(),
            count: 0,
        }
    }

    pub const fn count(&self) -> u32 {
        self.count
    }

    pub fn allocate(&mut self) -> Register {
        if let Some(&register) = self.free.iter().next() {
            self.free.remove(&register);
            register
        } else {
            let register = Register(self.count);
            self.count += 1;
            register
        }
    }

    pub fn free(&mut self, register: Register) {
        self.free.insert(register);
    }
}
