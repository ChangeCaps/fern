use crate::{
    ast,
    compiler::Type,
    error::Error,
    id::{BlockId, FunctionId, ModuleId},
    ir::{
        Blocks, Immediate, InstructionBuilder, MemoryType, Register, RegisterAllocator, Stack,
        StackAllocation, StackSlot,
    },
};

use super::{Declarations, FunctionSignatures, Functions, Types, Value, ValueKind};

pub enum ControlFlow {
    None,
    Return,
}

pub type ErrorFlow = Result<ControlFlow, Error>;

impl From<Error> for ErrorFlow {
    fn from(error: Error) -> Self {
        Self::Err(error)
    }
}

fn err<T>(error: Error) -> Result<T, ErrorFlow> {
    Err(ErrorFlow::Err(error))
}

pub struct FunctionVariable {
    pub ident: ast::Ident,
    pub ty: Type,
    pub stack_slot: StackSlot,
}

pub struct FunctionContext<'a> {
    pub types: &'a mut Types,
    pub signatures: &'a mut FunctionSignatures,
    pub blocks: &'a mut Blocks,
    pub module: ModuleId,
    pub stack: Stack,
    pub registers: RegisterAllocator,
    pub block: BlockId,
    pub variables: Vec<FunctionVariable>,
    pub return_type: Type,
}

impl<'a> FunctionContext<'a> {
    pub fn ins(&mut self) -> InstructionBuilder<'_> {
        InstructionBuilder {
            register_allocator: &mut self.registers,
            block: self.blocks.get_mut(self.block).unwrap(),
        }
    }

    pub fn set_block(&mut self, block: BlockId) {
        self.block = block;
    }

    pub fn free(&mut self, value: Value) {
        match value.kind {
            ValueKind::Register(register) => {
                self.registers.free(register);
            }
            _ => {}
        }
    }
}

pub struct FunctionCompiler<'a> {
    pub declarations: &'a Declarations,
    pub functions: &'a Functions,
}

impl<'a> FunctionCompiler<'a> {
    pub fn new(declarations: &'a Declarations, functions: &'a Functions) -> Self {
        Self {
            declarations,
            functions,
        }
    }

    pub fn compile_value(
        &self,
        ctx: &mut FunctionContext<'_>,
        value: Value,
    ) -> Result<Register, Error> {
        match value.kind {
            ValueKind::Register(register) => Ok(register),
            ValueKind::Stack(slot) => match value.ty {
                Type::Void => Ok(ctx.ins().iconst(Immediate::ZERO, MemoryType::Usize)),
                Type::Memory(ty) => Ok(ctx.ins().stack_load(slot, ty, 0)),
                Type::Struct(_) => unimplemented!(),
                Type::Reference(_) | Type::Function(_) => {
                    Ok(ctx.ins().stack_load(slot, MemoryType::Usize, 0))
                }
            },
            ValueKind::Function(id) => Ok(ctx.ins().func_addr(id, MemoryType::Usize)),
        }
    }

    pub fn stack_store_value(
        &self,
        ctx: &mut FunctionContext<'_>,
        stack_slot: StackSlot,
        value: Value,
    ) -> Result<(), Error> {
        let register = self.compile_value(ctx, value)?;

        match value.ty {
            Type::Void => {}
            Type::Memory(ty) => ctx.ins().stack_store(register, stack_slot, ty, 0),
            Type::Struct(_) => unimplemented!(),
            Type::Reference(_) | Type::Function(_) => {
                ctx.ins()
                    .stack_store(register, stack_slot, MemoryType::Usize, 0)
            }
        }

        Ok(())
    }

    pub fn store_value(
        &self,
        ctx: &mut FunctionContext<'_>,
        dst: Register,
        value: Value,
    ) -> Result<(), Error> {
        let register = self.compile_value(ctx, value)?;

        match value.ty {
            Type::Void => {}
            Type::Memory(ty) => ctx.ins().store(dst, register, ty, 0),
            Type::Struct(_) => unimplemented!(),
            Type::Reference(_) | Type::Function(_) => {
                ctx.ins().store(dst, register, MemoryType::Usize, 0)
            }
        }

        Ok(())
    }

    pub fn compile_paren(
        &self,
        ctx: &mut FunctionContext<'_>,
        ast: &ast::ParenExpression,
    ) -> Result<Value, ErrorFlow> {
        self.compile_expression(ctx, &ast.expression)
    }

    pub fn compile_literal(
        &self,
        ctx: &mut FunctionContext<'_>,
        ast: &ast::LiteralExpression,
    ) -> Result<Value, ErrorFlow> {
        match ast {
            ast::LiteralExpression::String(_) => unimplemented!(),
            ast::LiteralExpression::Integer(integer) => {
                let value = integer.integer().value();
                let register = ctx.ins().iconst(value as i32, MemoryType::I32);

                Ok(Value::new(Type::Memory(MemoryType::I32), register))
            }
        }
    }

    pub fn compile_path(
        &self,
        ctx: &mut FunctionContext<'_>,
        ast: &ast::Path,
    ) -> Result<Value, ErrorFlow> {
        if let Some(ident) = ast.as_ident() {
            let variable = ctx.variables.iter().rev().find(|var| var.ident == *ident);

            if let Some(variable) = variable {
                return Ok(Value::new(variable.ty, variable.stack_slot));
            }
        }

        let module_id = self.declarations.canonicalize_module(ctx.module, ast)?;
        let module = &self.declarations.modules[&module_id];

        if let Some(ident) = ast.get_ident() {
            if let Some(&function_id) = module.functions.get(ident) {
                let function = &self.functions.functions[&function_id];

                return Ok(Value::new(Type::Function(function.signature), function_id));
            }
        }

        err(Error::new(format!("{:?} not defined", ast)))
    }

    pub fn compile_call(
        &self,
        ctx: &mut FunctionContext,
        ast: &ast::CallExpression,
    ) -> Result<Value, ErrorFlow> {
        let function = self.compile_expression(ctx, &ast.function)?;

        let signature_id = if let Type::Function(id) = function.ty {
            id
        } else {
            return err(Error::new("Cannot call"));
        };

        let mut arguments = Vec::with_capacity(ast.arguments.len());
        let mut types = Vec::with_capacity(ast.arguments.len());

        for argument in ast.arguments.iter() {
            let value = self.compile_expression(ctx, &argument)?;
            let register = self.compile_value(ctx, value)?;

            types.push(ctx.types.get_type_id(&value.ty));
            arguments.push(register);
        }

        let signature = ctx.signatures.get_signature(signature_id);
        let return_type = ctx.types.get_type(signature.return_type);

        if types != signature.arguments {
            return err(Error::new("Arguments don't match function signature"));
        }

        match function.kind {
            ValueKind::Function(function_id) => {
                let register = ctx.ins().call(function_id, arguments);

                Ok(Value::new(return_type, register))
            }
            _ => unimplemented!(),
        }
    }

    pub fn compile_unary(
        &self,
        ctx: &mut FunctionContext<'_>,
        ast: &ast::UnaryExpression,
    ) -> Result<Value, ErrorFlow> {
        let value = self.compile_expression(ctx, &ast.expression)?;

        match ast.operator {
            ast::UnaryOperator::Reference(_) => match value.kind {
                ValueKind::Stack(slot) => {
                    let ty = ctx.types.get_type_id(&value.ty);
                    let register = ctx.ins().stack_addr(MemoryType::Usize, slot);

                    ctx.free(value);
                    Ok(Value::new(Type::Reference(ty), register))
                }
                _ => err(Error::new("Cannot reference")),
            },
            ast::UnaryOperator::Dereference(_) => {
                if let Type::Reference(inner) = value.ty {
                    match value.kind {
                        ValueKind::Register(register) => {
                            let ty = ctx.types.get_type(inner);

                            let kind = match ty {
                                Type::Void => {
                                    let register = ctx.registers.allocate();
                                    ctx.free(value);
                                    register
                                }
                                Type::Memory(ty) => {
                                    let register = ctx.ins().load(register, ty, 0);
                                    ctx.free(value);
                                    register
                                }
                                Type::Struct(_) => register,
                                Type::Reference(_) | Type::Function(_) => {
                                    let register = ctx.ins().load(register, MemoryType::Usize, 0);
                                    ctx.free(value);
                                    register
                                }
                            };

                            Ok(Value::new(ty, kind))
                        }
                        ValueKind::Stack(stack_slot) => {
                            let ty = ctx.types.get_type(inner);

                            let kind = match ty {
                                Type::Void => {
                                    let register = ctx.registers.allocate();
                                    ValueKind::from(register)
                                }
                                Type::Memory(ty) => {
                                    let register = ctx.ins().stack_load(stack_slot, ty, 0);
                                    ValueKind::from(register)
                                }
                                Type::Struct(_) => ValueKind::from(stack_slot),
                                Type::Reference(_) | Type::Function(_) => {
                                    let register =
                                        ctx.ins().stack_load(stack_slot, MemoryType::Usize, 0);
                                    ValueKind::from(register)
                                }
                            };

                            ctx.free(value);
                            Ok(Value::new(ty, kind))
                        }
                        _ => unimplemented!(),
                    }
                } else {
                    err(Error::new("Cannot dereference"))
                }
            }
            ast::UnaryOperator::Negate(_) => todo!(),
        }
    }

    pub fn compile_binary(
        &self,
        ctx: &mut FunctionContext<'_>,
        ast: &ast::BinaryExpression,
    ) -> Result<Value, ErrorFlow> {
        let lhs = self.compile_expression(ctx, &ast.lhs)?;
        let rhs = self.compile_expression(ctx, &ast.rhs)?;

        let lhs_val = self.compile_value(ctx, lhs)?;
        let rhs_val = self.compile_value(ctx, rhs)?;

        match ast.operator {
            ast::BinaryOperator::Add(_) if rhs.ty == lhs.ty && lhs.ty.is_integer() => {
                let dst = ctx.ins().add(lhs_val, rhs_val);

                ctx.free(lhs);
                ctx.free(rhs);

                Ok(Value::new(lhs.ty, dst))
            }
            _ => unimplemented!(),
        }
    }

    pub fn compile_return(
        &self,
        ctx: &mut FunctionContext<'_>,
        ast: &ast::ReturnExpression,
    ) -> Result<Value, ErrorFlow> {
        let value = self.compile_expression(ctx, &ast.expression)?;
        let register = self.compile_value(ctx, value)?;

        if value.ty != ctx.return_type {
            return err(Error::new("Invalid return type"));
        }

        ctx.ins().ret(register);

        Err(Ok(ControlFlow::Return))
    }

    pub fn compile_expression(
        &self,
        ctx: &mut FunctionContext<'_>,
        expression: &ast::Expression,
    ) -> Result<Value, ErrorFlow> {
        match expression {
            ast::Expression::Paren(paren) => self.compile_paren(ctx, paren),
            ast::Expression::Literal(literal) => self.compile_literal(ctx, literal),
            ast::Expression::Path(path) => self.compile_path(ctx, path),
            ast::Expression::Call(call) => self.compile_call(ctx, call),
            ast::Expression::Unary(unary) => self.compile_unary(ctx, unary),
            ast::Expression::Binary(binary) => self.compile_binary(ctx, binary),
            ast::Expression::Return(ast) => self.compile_return(ctx, ast),
        }
    }

    pub fn compile_let(
        &self,
        ctx: &mut FunctionContext<'_>,
        ast: &ast::LetStatement,
    ) -> Result<ControlFlow, Error> {
        let (value, ty) = if let Some(ref value) = ast.value {
            let value = match self.compile_expression(ctx, &value.expression) {
                Ok(value) => value,
                Err(flow) => return flow,
            };

            if let Some(ref ty) = ast.ty {
                let ty = self.declarations.resolve_type(ctx.types, &ty.ty)?;

                if value.ty != ty {
                    return Err(Error::new("Must match type defined"));
                }
            }

            (Some(value), value.ty)
        } else {
            let ast = ast.ty.as_ref().ok_or(Error::new("Type must be defined"))?;

            let ty = self.declarations.resolve_type(ctx.types, &ast.ty)?;

            (None, ty)
        };

        let type_id = ctx.types.get_type_id(&ty);
        let stack_slot = ctx.stack.allocate(StackAllocation { ty: type_id });

        ctx.variables.push(FunctionVariable {
            ident: ast.ident.clone(),
            ty,
            stack_slot,
        });

        if let Some(value) = value {
            self.stack_store_value(ctx, stack_slot, value)?;
            ctx.free(value);
        }

        Ok(ControlFlow::None)
    }

    pub fn compile_statement(
        &self,
        ctx: &mut FunctionContext<'_>,
        statement: &ast::Statement,
    ) -> Result<ControlFlow, Error> {
        match statement {
            ast::Statement::Noop(_) => Ok(ControlFlow::None),
            ast::Statement::Expression(expression) => {
                match self.compile_expression(ctx, &expression.expression) {
                    Ok(value) => {
                        ctx.free(value);

                        Ok(ControlFlow::None)
                    }
                    Err(error_flow) => error_flow,
                }
            }
            ast::Statement::Let(ast) => self.compile_let(ctx, ast),
        }
    }

    pub fn compile_function(
        &self,
        blocks: &mut Blocks,
        types: &mut Types,
        signatures: &mut FunctionSignatures,
        id: FunctionId,
    ) -> Result<(), Error> {
        let declaration = &self.declarations.functions[&id];
        let function = &self.functions.functions[&id];
        let return_type = types.get_type(function.return_type);

        let entry_point = blocks.create();

        let mut ctx = FunctionContext {
            types,
            signatures,
            blocks,
            module: declaration.module,
            registers: RegisterAllocator::new(),
            stack: Stack::new(),
            block: entry_point,
            variables: Vec::new(),
            return_type,
        };

        let mut returned = false;

        for statement in declaration.ast.block.iter() {
            let control_flow = self.compile_statement(&mut ctx, statement)?;

            match control_flow {
                ControlFlow::Return => {
                    returned = true;

                    break;
                }
                ControlFlow::None => {}
            }
        }

        if !returned && return_type != Type::Void {
            Err(Error::new("Function must return"))
        } else {
            Ok(())
        }
    }

    pub fn compile_program(
        &self,
        blocks: &mut Blocks,
        types: &mut Types,
        signatures: &mut FunctionSignatures,
    ) -> Result<(), Error> {
        for id in self.declarations.functions.keys().copied() {
            self.compile_function(blocks, types, signatures, id)?;
        }

        Ok(())
    }
}
