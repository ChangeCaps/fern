use std::collections::HashMap;

use crate::{
    ast,
    error::Error,
    id::{FunctionId, FunctionIds, FunctionSignatureId, FunctionSignatureIds, TypeId},
};

use super::{Declarations, Type, Types};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
    pub arguments: Vec<TypeId>,
    pub return_type: TypeId,
}

#[derive(Clone, Debug, Default)]
pub struct FunctionSignatures {
    ids: FunctionSignatureIds,
    id_to_signature: HashMap<FunctionSignatureId, FunctionSignature>,
    signature_to_id: HashMap<FunctionSignature, FunctionSignatureId>,
}

impl FunctionSignatures {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_id(&mut self, signature: &FunctionSignature) -> FunctionSignatureId {
        if let Some(&id) = self.signature_to_id.get(signature) {
            id
        } else {
            let id = self.ids.generate();

            self.id_to_signature.insert(id, signature.clone());
            self.signature_to_id.insert(signature.clone(), id);

            id
        }
    }

    pub fn get_signature(&self, id: FunctionSignatureId) -> &FunctionSignature {
        &self.id_to_signature[&id]
    }
}

#[derive(Clone, Debug)]
pub struct FunctionArgument {
    pub ident: ast::Ident,
    pub ty: TypeId,
}

impl FunctionArgument {
    pub fn from_ast(
        declarations: &Declarations,
        types: &mut Types,
        ast: &ast::FunctionArgument,
    ) -> Result<Self, Error> {
        Ok(Self {
            ident: ast.ident.clone(),
            ty: declarations.resolve_type_id(types, &ast.ty.ty)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub arguments: Vec<FunctionArgument>,
    pub return_type: TypeId,
    pub signature: FunctionSignatureId,
}

impl Function {
    pub fn from_ast(
        declarations: &Declarations,
        types: &mut Types,
        signatures: &mut FunctionSignatures,
        ast: &ast::FunctionDeclaration,
    ) -> Result<Self, Error> {
        let mut arguments = Vec::with_capacity(ast.args.len());

        for arg in ast.args.iter() {
            arguments.push(FunctionArgument::from_ast(declarations, types, arg)?);
        }

        let return_type = if let Some(ref return_type) = ast.return_type {
            declarations.resolve_type_id(types, &return_type.ty)?
        } else {
            types.get_type_id(&Type::Void)
        };

        let signature = FunctionSignature {
            arguments: arguments.iter().map(|arg| arg.ty).collect(),
            return_type,
        };

        Ok(Self {
            arguments,
            return_type,
            signature: signatures.get_id(&signature),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct Functions {
    pub ids: FunctionIds,
    pub functions: HashMap<FunctionId, Function>,
}

impl Functions {
    pub fn new(
        declarations: &Declarations,
        types: &mut Types,
        signatures: &mut FunctionSignatures,
    ) -> Result<Self, Error> {
        let mut this = Self::default();

        for (&id, function) in declarations.functions.iter() {
            this.insert_function(declarations, types, signatures, id, &function.ast)?;
        }

        Ok(this)
    }

    pub fn insert_function(
        &mut self,
        path_stage: &Declarations,
        types: &mut Types,
        signatures: &mut FunctionSignatures,
        id: FunctionId,
        function: &ast::FunctionDeclaration,
    ) -> Result<(), Error> {
        let function = Function::from_ast(path_stage, types, signatures, function)?;
        self.functions.insert(id, function);

        Ok(())
    }
}
