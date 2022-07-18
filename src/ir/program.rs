use std::collections::HashMap;

use crate::{
    compiler::{FunctionSignatures, Types},
    id::FunctionId,
};

use super::{Blocks, Function};

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub types: Types,
    pub signatures: FunctionSignatures,
    pub blocks: Blocks,
    pub functions: HashMap<FunctionId, Function>,
}
