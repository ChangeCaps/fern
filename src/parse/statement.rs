use crate::{
    ast::{
        ExpressionStatement, Keyword, LetStatement, LetStatementValue, Statement, Symbol, Token,
    },
    error::{Error, Expected},
};

use super::{Parse, Parser};

impl Parse for LetStatementValue {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            equal: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}

impl Parse for LetStatement {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let _let = parser.parse()?;
        let ident = parser.parse()?;

        let ty = if let Token::Symbol(Symbol::Colon) = parser.peek_token()? {
            Some(parser.parse()?)
        } else {
            None
        };

        let value = if let Token::Symbol(Symbol::Equal) = parser.peek_token()? {
            Some(parser.parse()?)
        } else {
            None
        };

        Ok(Self {
            _let,
            ident,
            ty,
            value,
            semi_colon: parser.parse()?,
        })
    }
}

impl Parse for ExpressionStatement {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            expression: parser.parse()?,
            semi_colon: parser.parse()?,
        })
    }
}

impl Parse for Statement {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Symbol(Symbol::SemiColon) => Ok(Self::Noop(parser.parse()?)),
            Token::Ident(_)
            | Token::String(_)
            | Token::Integer(_)
            | Token::Keyword(Keyword::Return)
            | Token::Symbol(Symbol::OpenParen)
            | Token::Symbol(Symbol::And)
            | Token::Symbol(Symbol::Asterisk)
            | Token::Symbol(Symbol::Minus) => Ok(Self::Expression(parser.parse()?)),
            Token::Keyword(Keyword::Let) => Ok(Self::Let(parser.parse()?)),
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
