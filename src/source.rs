#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SourceId(usize);

impl SourceId {
    pub const fn null() -> Self {
        Self(0)
    }
}

pub struct Source {}

pub struct Sources {}
