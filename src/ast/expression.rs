use crate::ast;

#[derive(Clone, Debug)]
pub enum LiteralExpression {
    String(ast::StringLiteral),
    Integer(ast::IntegerLiteral),
}

#[derive(Clone, Debug)]
pub struct ParenExpression {
    pub open: ast::OpenParen,
    pub expression: Box<ast::Expression>,
    pub close: ast::CloseParen,
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub function: Box<ast::Expression>,
    pub open: ast::OpenParen,
    pub arguments: ast::Punctuated<ast::Expression, ast::Colon>,
    pub close: ast::CloseParen,
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Reference(ast::And),
    Dereference(ast::Asterisk),
    Negate(ast::Minus),
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: ast::UnaryOperator,
    pub expression: Box<ast::Expression>,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Add(ast::Plus),
    Sub(ast::Minus),
    Mul(ast::Asterisk),
    Div(ast::Slash),
    LogicalAnd(ast::AndAnd),
    LogicalOr(ast::OrOr),
    BinaryAnd(ast::And),
    BinaryOr(ast::Or),
    BitShiftRight(ast::ShiftRight),
    BitShiftLeft(ast::ShiftLeft),
}

impl BinaryOperator {
    pub fn precedence(&self) -> u32 {
        match self {
            Self::Mul(_) | Self::Div(_) => 12,
            Self::Add(_) | Self::Sub(_) => 11,
            Self::BitShiftLeft(_) | Self::BitShiftRight(_) => 10,
            Self::LogicalAnd(_) => 4,
            Self::LogicalOr(_) => 3,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub lhs: Box<ast::Expression>,
    pub operator: ast::BinaryOperator,
    pub rhs: Box<ast::Expression>,
}

#[derive(Clone, Debug)]
pub struct ReturnExpression {
    pub _return: ast::Return,
    pub expression: Box<ast::Expression>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Paren(ast::ParenExpression),
    Literal(ast::LiteralExpression),
    Path(ast::Path),
    Call(ast::CallExpression),
    Unary(ast::UnaryExpression),
    Binary(ast::BinaryExpression),
    Return(ast::ReturnExpression),
}
