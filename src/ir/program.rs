use crate::compiler::{FunctionSignatures, Types};

use super::Blocks;

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub types: Types,
    pub signatures: FunctionSignatures,
    pub blocks: Blocks,
}
