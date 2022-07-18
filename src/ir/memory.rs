use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemoryType {
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    Bool,
}

impl MemoryType {
    pub const fn is_integer(&self) -> bool {
        match self {
            Self::Bool => false,
            _ => true,
        }
    }

    pub const fn size(&self, ptr_size: u64) -> u64 {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 => 4,
            Self::U64 | Self::I64 => 8,
            Self::Usize | Self::Isize => ptr_size,
            Self::Bool => 4,
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Immediate([u8; 8]);

impl Immediate {
    pub const ZERO: Self = Self([0; 8]);
}

impl From<i32> for Immediate {
    fn from(value: i32) -> Self {
        let bytes = value.to_le_bytes();
        Self([0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl Into<i32> for Immediate {
    fn into(self) -> i32 {
        i32::from_le_bytes([self.0[4], self.0[5], self.0[6], self.0[7]])
    }
}
