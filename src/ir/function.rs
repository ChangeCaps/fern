use crate::id::{BlockId, FunctionSignatureId};

use super::Stack;

#[derive(Clone, Debug)]
pub struct Function {
    pub label: Option<String>,
    pub signature: FunctionSignatureId,
    pub blocks: Vec<BlockId>,
    pub stack: Stack,
}
