use crate::ast;

#[derive(Clone, Debug)]
pub struct Block {
    pub open: ast::OpenBrace,
    pub statements: Vec<ast::Statement>,
    pub close: ast::CloseBrace,
}

impl Block {
    pub fn iter(&self) -> impl Iterator<Item = &ast::Statement> {
        self.statements.iter()
    }
}
