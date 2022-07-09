use std::collections::HashMap;

use crate::id::{BlockId, BlockIds};

use super::Instruction;

#[derive(Clone, Debug)]
pub struct Block {
    instructions: Vec<Instruction>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Blocks {
    block_ids: BlockIds,
    blocks: HashMap<BlockId, Block>,
}

impl Blocks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&mut self) -> BlockId {
        let id = self.block_ids.generate();
        self.blocks.insert(id, Block::new());
        id
    }

    pub fn get(&self, id: BlockId) -> Option<&Block> {
        self.blocks.get(&id)
    }

    pub fn get_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.blocks.get_mut(&id)
    }

    pub fn push(&mut self, id: BlockId, instruction: Instruction) {
        self.get_mut(id).unwrap().push(instruction);
    }
}
