use std::collections::HashMap;

use crate::id::{BlockId, FunctionId, FunctionSignatureId};

use super::Stack;

#[derive(Clone, Debug)]
pub struct Function {
    pub label: Option<String>,
    pub signature: FunctionSignatureId,
    pub blocks: Vec<BlockId>,
    pub stack: Stack,
}

#[derive(Clone, Debug, Default)]
pub struct Functions {
    pub functions: HashMap<FunctionId, Function>,
}

impl Functions {
    pub fn insert(&mut self, id: FunctionId, function: Function) {
        self.functions.insert(id, function);
    }
}
