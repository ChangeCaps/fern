use crate::ast;

#[derive(Clone, Debug)]
pub enum IntegerType {
    I8(ast::I8),
    U8(ast::U8),
    I16(ast::I16),
    U16(ast::U16),
    I32(ast::I32),
    U32(ast::U32),
    I64(ast::I64),
    U64(ast::U64),
}

#[derive(Clone, Debug)]
pub struct ReferenceType {
    pub and: ast::And,
    pub ty: Box<ast::Type>,
}

#[derive(Clone, Debug)]
pub enum Type {
    Void(ast::Void),
    Boolean(ast::Bool),
    Integer(ast::IntegerType),
    Path(ast::Path),
    Reference(ast::ReferenceType),
}

#[derive(Clone, Debug)]
pub struct TypeDeclaration {
    pub colon: ast::Colon,
    pub ty: ast::Type,
}
