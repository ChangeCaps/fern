use crate::compiler::{FunctionSignatures, Types};

use super::{Blocks, Functions};

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub types: Types,
    pub signatures: FunctionSignatures,
    pub blocks: Blocks,
    pub functions: Functions,
}
