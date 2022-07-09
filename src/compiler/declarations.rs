use std::collections::HashMap;

use crate::{
    ast,
    error::Error,
    id::{FunctionId, FunctionIds, ModuleId, ModuleIds, TypeId},
    ir::MemoryType,
};

use super::{Type, Types};

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub ast: ast::FunctionDeclaration,
    pub module: ModuleId,
}

#[derive(Clone, Debug, Default)]
pub struct Module {
    pub super_module: Option<ModuleId>,
    pub sub_modules: HashMap<ast::Ident, ModuleId>,
    pub functions: HashMap<ast::Ident, FunctionId>,
}

#[derive(Debug)]
pub struct Declarations {
    pub module_ids: ModuleIds,
    pub function_ids: FunctionIds,
    pub base_module: ModuleId,
    pub modules: HashMap<ModuleId, Module>,
    pub functions: HashMap<FunctionId, FunctionDeclaration>,
}

impl Declarations {
    pub fn new() -> Self {
        let mut module_ids = ModuleIds::default();
        let base_module = module_ids.generate();

        let mut modules = HashMap::new();
        modules.insert(base_module, Default::default());

        Self {
            module_ids,
            function_ids: Default::default(),
            base_module,
            modules,
            functions: Default::default(),
        }
    }

    pub fn from_program(program: ast::Program) -> Result<Self, Error> {
        let mut this = Declarations::new();

        for declaration in program.declarations {
            this.insert_declaration(this.base_module, declaration)?;
        }

        Ok(this)
    }

    pub fn resolve_type(&self, types: &mut Types, ty: &ast::Type) -> Result<Type, Error> {
        match ty {
            ast::Type::Void(_) => Ok(Type::Void),
            ast::Type::Boolean(_) => Ok(Type::Memory(MemoryType::Bool)),
            ast::Type::Integer(integer) => match integer {
                ast::IntegerType::I32(_) => Ok(Type::Memory(MemoryType::I32)),
                ast::IntegerType::U32(_) => Ok(Type::Memory(MemoryType::U32)),
            },
            ast::Type::Path(_) => todo!(),
            ast::Type::Reference(inner) => {
                let ty = self.resolve_type(types, &inner.ty)?;
                let id = types.get_type_id(&ty);
                Ok(Type::Reference(id))
            }
        }
    }

    pub fn resolve_type_id(&self, types: &mut Types, ty: &ast::Type) -> Result<TypeId, Error> {
        let ty = self.resolve_type(types, ty)?;
        Ok(types.get_type_id(&ty))
    }

    pub fn insert_declaration(
        &mut self,
        module_id: ModuleId,
        declaration: ast::Declaration,
    ) -> Result<(), Error> {
        let module = self.modules.get_mut(&module_id).unwrap();

        match declaration {
            ast::Declaration::Function(function) => {
                let function_id = self.function_ids.generate();

                module.functions.insert(function.ident.clone(), function_id);
                self.functions.insert(
                    function_id,
                    FunctionDeclaration {
                        ast: function,
                        module: module_id,
                    },
                );
            }
        }

        Ok(())
    }

    pub fn canonicalize_module(
        &self,
        mut module_id: ModuleId,
        path: &ast::Path,
    ) -> Result<ModuleId, Error> {
        if path.is_absolute() {
            module_id = self.base_module;
        }

        for segment in path.iter_modules() {
            let module = &self.modules[&module_id];

            match segment {
                ast::PathSegment::Super => {
                    if let Some(super_module) = module.super_module {
                        module_id = super_module;
                    } else {
                        return Err(Error::new("invalid path"));
                    }
                }
                ast::PathSegment::Ident(ident) => {
                    if let Some(&sub_module) = module.sub_modules.get(ident) {
                        module_id = sub_module;
                    } else {
                        return Err(Error::new("invalid path"));
                    }
                }
            }
        }

        Ok(module_id)
    }
}
