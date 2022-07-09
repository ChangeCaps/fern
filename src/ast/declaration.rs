use crate::ast;

#[derive(Clone, Debug)]
pub struct ReturnType {
    pub arrow: ast::Arrow,
    pub ty: ast::Type,
}

#[derive(Clone, Debug)]
pub struct FunctionArgument {
    pub ident: ast::Ident,
    pub ty: ast::TypeDeclaration,
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub _fn: ast::Fn,
    pub ident: ast::Ident,
    pub open: ast::OpenParen,
    pub args: ast::Punctuated<ast::FunctionArgument, ast::Comma>,
    pub close: ast::CloseParen,
    pub return_type: Option<ast::ReturnType>,
    pub block: ast::Block,
}

#[derive(Clone, Debug)]
pub enum Declaration {
    Function(FunctionDeclaration),
}
