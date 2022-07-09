use crate::{
    ast::{
        BinaryExpression, BinaryOperator, CallExpression, Expression, Keyword, LiteralExpression,
        ParenExpression, Punctuated, ReturnExpression, Symbol, Token, UnaryExpression,
        UnaryOperator,
    },
    error::{Error, Expected},
};

use super::{Parse, Parser};

impl Parse for LiteralExpression {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Integer(_) => Ok(LiteralExpression::Integer(parser.parse()?)),
            Token::String(_) => Ok(LiteralExpression::String(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[Expected::String, Expected::Integer],
                tok,
                parser.next_span()?,
            )),
        }
    }
}

impl Parse for ParenExpression {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            open: parser.parse()?,
            expression: parser.parse()?,
            close: parser.parse()?,
        })
    }
}

impl Parse for UnaryOperator {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Symbol(Symbol::And) => Ok(Self::Reference(parser.parse()?)),
            Token::Symbol(Symbol::Asterisk) => Ok(Self::Dereference(parser.parse()?)),
            Token::Symbol(Symbol::Minus) => Ok(Self::Negate(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[
                    Expected::Symbol(Symbol::And),
                    Expected::Symbol(Symbol::Asterisk),
                    Expected::Symbol(Symbol::Minus),
                ],
                tok,
                parser.next_span()?,
            )),
        }
    }
}

impl Parse for BinaryOperator {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Symbol(Symbol::Plus) => Ok(Self::Add(parser.parse()?)),
            Token::Symbol(Symbol::Minus) => Ok(Self::Sub(parser.parse()?)),
            Token::Symbol(Symbol::Asterisk) => Ok(Self::Mul(parser.parse()?)),
            Token::Symbol(Symbol::Slash) => Ok(Self::Div(parser.parse()?)),
            Token::Symbol(Symbol::AndAnd) => Ok(Self::LogicalAnd(parser.parse()?)),
            Token::Symbol(Symbol::OrOr) => Ok(Self::LogicalOr(parser.parse()?)),
            Token::Symbol(Symbol::And) => Ok(Self::BinaryAnd(parser.parse()?)),
            Token::Symbol(Symbol::Or) => Ok(Self::BinaryOr(parser.parse()?)),
            Token::Symbol(Symbol::ShiftRight) => Ok(Self::BitShiftRight(parser.parse()?)),
            Token::Symbol(Symbol::ShiftLeft) => Ok(Self::BitShiftLeft(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[
                    Expected::Symbol(Symbol::Plus),
                    Expected::Symbol(Symbol::Minus),
                    Expected::Symbol(Symbol::Asterisk),
                    Expected::Symbol(Symbol::Slash),
                    Expected::Symbol(Symbol::AndAnd),
                    Expected::Symbol(Symbol::OrOr),
                    Expected::Symbol(Symbol::And),
                    Expected::Symbol(Symbol::Or),
                    Expected::Symbol(Symbol::ShiftRight),
                    Expected::Symbol(Symbol::ShiftLeft),
                ],
                tok,
                parser.next_span()?,
            )),
        }
    }
}

impl Parse for BinaryExpression {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            lhs: parser.parse()?,
            operator: parser.parse()?,
            rhs: parser.parse()?,
        })
    }
}

impl Parse for ReturnExpression {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            _return: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}

fn parse_term_expression(parser: &mut Parser) -> Result<Expression, Error> {
    match parser.peek_token()? {
        Token::Integer(_) | Token::String(_) => Ok(Expression::Literal(parser.parse()?)),
        Token::Symbol(Symbol::OpenParen) => Ok(Expression::Paren(parser.parse()?)),
        Token::Ident(_) => Ok(Expression::Path(parser.parse()?)),
        tok => Err(Error::expected_any(
            &[
                Expected::Ident,
                Expected::String,
                Expected::Integer,
                Expected::Symbol(Symbol::OpenParen),
            ],
            tok,
            parser.next_span()?,
        )),
    }
}

fn parse_call_expression(parser: &mut Parser) -> Result<Expression, Error> {
    let function = parse_term_expression(parser)?;

    match parser.peek_token()? {
        Token::Symbol(Symbol::OpenParen) => Ok(Expression::Call(CallExpression {
            function: Box::new(function),
            open: parser.parse()?,
            arguments: Punctuated::parse_terminated(parser, Token::Symbol(Symbol::CloseParen))?,
            close: parser.parse()?,
        })),
        _ => Ok(function),
    }
}

fn parse_unary_expression(parser: &mut Parser) -> Result<Expression, Error> {
    match parser.peek_token()? {
        Token::Symbol(Symbol::And)
        | Token::Symbol(Symbol::Asterisk)
        | Token::Symbol(Symbol::Minus) => {
            let operator = parser.parse::<UnaryOperator>()?;

            Ok(Expression::Unary(UnaryExpression {
                operator,
                expression: Box::new(parse_unary_expression(parser)?),
            }))
        }
        _ => parse_call_expression(parser),
    }
}

fn parse_binary_expression(parser: &mut Parser) -> Result<Expression, Error> {
    let lhs = parse_unary_expression(parser)?;

    match parser.peek_token()? {
        Token::Symbol(Symbol::Plus)
        | Token::Symbol(Symbol::Minus)
        | Token::Symbol(Symbol::Asterisk)
        | Token::Symbol(Symbol::Slash)
        | Token::Symbol(Symbol::AndAnd)
        | Token::Symbol(Symbol::OrOr)
        | Token::Symbol(Symbol::And)
        | Token::Symbol(Symbol::Or)
        | Token::Symbol(Symbol::ShiftRight)
        | Token::Symbol(Symbol::ShiftLeft) => {
            let operator = parser.parse::<BinaryOperator>()?;
            let rhs = parse_binary_expression(parser)?;

            if let Expression::Binary(rhs) = rhs {
                if operator.precedence() >= rhs.operator.precedence() {
                    Ok(Expression::Binary(BinaryExpression {
                        lhs: Box::new(Expression::Binary(BinaryExpression {
                            lhs: Box::new(lhs),
                            operator,
                            rhs: rhs.lhs,
                        })),
                        operator: rhs.operator,
                        rhs: rhs.rhs,
                    }))
                } else {
                    Ok(Expression::Binary(BinaryExpression {
                        lhs: Box::new(lhs),
                        operator,
                        rhs: Box::new(Expression::Binary(rhs)),
                    }))
                }
            } else {
                Ok(Expression::Binary(BinaryExpression {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                }))
            }
        }
        _ => Ok(lhs),
    }
}

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Ident(_)
            | Token::String(_)
            | Token::Integer(_)
            | Token::Symbol(Symbol::OpenParen)
            | Token::Symbol(Symbol::And)
            | Token::Symbol(Symbol::Asterisk)
            | Token::Symbol(Symbol::Minus) => parse_binary_expression(parser),
            Token::Keyword(Keyword::Return) => Ok(Self::Return(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[
                    Expected::Ident,
                    Expected::String,
                    Expected::Integer,
                    Expected::Keyword(Keyword::Return),
                    Expected::Symbol(Symbol::OpenParen),
                    Expected::Symbol(Symbol::And),
                    Expected::Symbol(Symbol::Asterisk),
                    Expected::Symbol(Symbol::Minus),
                ],
                tok,
                parser.next_span()?,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_expression() {}
}
