use std::collections::HashMap;

use crate::{
    ast,
    id::{FunctionSignatureId, StructId, TypeId, TypeIds},
    ir::MemoryType,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Void,
    Memory(MemoryType),
    Struct(StructId),
    Reference(TypeId),
    Function(FunctionSignatureId),
}

impl Type {
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Memory(ty) => ty.is_integer(),
            _ => false,
        }
    }

    pub fn is_reference(&self) -> bool {
        match self {
            Self::Reference(_) => true,
            _ => false,
        }
    }
}

impl From<MemoryType> for Type {
    fn from(ty: MemoryType) -> Self {
        Self::Memory(ty)
    }
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub ident: ast::Ident,
}

#[derive(Clone, Debug, Default)]
pub struct Types {
    pub type_ids: TypeIds,
    pub id_to_type: HashMap<TypeId, Type>,
    pub type_to_id: HashMap<Type, TypeId>,
    pub structs: HashMap<StructId, Struct>,
}

impl Types {
    pub fn get_type_id(&mut self, ty: &Type) -> TypeId {
        if let Some(&id) = self.type_to_id.get(ty) {
            id
        } else {
            let id = self.type_ids.generate();

            self.id_to_type.insert(id, ty.clone());
            self.type_to_id.insert(ty.clone(), id);

            id
        }
    }

    pub fn get_type(&self, id: TypeId) -> Type {
        self.id_to_type[&id]
    }
}
