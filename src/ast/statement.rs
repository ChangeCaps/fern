use crate::ast;

#[derive(Clone, Debug)]
pub struct LetStatementValue {
    pub equal: ast::Equal,
    pub expression: ast::Expression,
}

#[derive(Clone, Debug)]
pub struct LetStatement {
    pub _let: ast::Let,
    pub ident: ast::Ident,
    pub ty: Option<ast::TypeDeclaration>,
    pub value: Option<ast::LetStatementValue>,
    pub semi_colon: ast::SemiColon,
}

#[derive(Clone, Debug)]
pub struct ExpressionStatement {
    pub expression: ast::Expression,
    pub semi_colon: ast::SemiColon,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Noop(ast::SemiColon),
    Expression(ast::ExpressionStatement),
    Let(ast::LetStatement),
}
