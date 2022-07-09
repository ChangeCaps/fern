use crate::ast;

#[derive(Clone, Debug)]
pub struct Program {
    pub declarations: Vec<ast::Declaration>,
}
