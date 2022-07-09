use crate::{
    ast::{IntegerType, Keyword, ReferenceType, Symbol, Token, Type, TypeDeclaration},
    error::{Error, Expected},
};

use super::{Parse, Parser};

impl Parse for IntegerType {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Keyword(Keyword::U32) => Ok(Self::U32(parser.parse()?)),
            Token::Keyword(Keyword::I32) => Ok(Self::I32(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[
                    Expected::Keyword(Keyword::U32),
                    Expected::Keyword(Keyword::I32),
                ],
                tok,
                parser.next_span()?,
            )),
        }
    }
}

impl Parse for ReferenceType {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            and: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}

impl Parse for Type {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Keyword(Keyword::Void) => Ok(Self::Void(parser.parse()?)),
            Token::Keyword(Keyword::Bool) => Ok(Self::Boolean(parser.parse()?)),
            Token::Keyword(Keyword::U32) | Token::Keyword(Keyword::I32) => {
                Ok(Self::Integer(parser.parse()?))
            }
            Token::Symbol(Symbol::And) => Ok(Self::Reference(parser.parse()?)),
            Token::Ident(_) => Ok(Self::Path(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[
                    Expected::Ident,
                    Expected::Symbol(Symbol::And),
                    Expected::Keyword(Keyword::Void),
                    Expected::Keyword(Keyword::Bool),
                    Expected::Keyword(Keyword::U32),
                    Expected::Keyword(Keyword::I32),
                ],
                tok,
                parser.next_span()?,
            )),
        }
    }
}

impl Parse for TypeDeclaration {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            colon: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}
