use crate::{
    ast::{IntegerType, Keyword, ReferenceType, Symbol, Token, Type, TypeDeclaration},
    error::{Error, Expected},
};

use super::{Parse, Parser};

impl Parse for IntegerType {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Keyword(Keyword::U8) => Ok(Self::U8(parser.parse()?)),
            Token::Keyword(Keyword::I8) => Ok(Self::I8(parser.parse()?)),
            Token::Keyword(Keyword::U16) => Ok(Self::U16(parser.parse()?)),
            Token::Keyword(Keyword::I16) => Ok(Self::I16(parser.parse()?)),
            Token::Keyword(Keyword::U32) => Ok(Self::U32(parser.parse()?)),
            Token::Keyword(Keyword::I32) => Ok(Self::I32(parser.parse()?)),
            Token::Keyword(Keyword::U64) => Ok(Self::U64(parser.parse()?)),
            Token::Keyword(Keyword::I64) => Ok(Self::I64(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[
                    Expected::Keyword(Keyword::U8),
                    Expected::Keyword(Keyword::I8),
                    Expected::Keyword(Keyword::U16),
                    Expected::Keyword(Keyword::I16),
                    Expected::Keyword(Keyword::U32),
                    Expected::Keyword(Keyword::I32),
                    Expected::Keyword(Keyword::U64),
                    Expected::Keyword(Keyword::I64),
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
            Token::Keyword(Keyword::U8)
            | Token::Keyword(Keyword::I8)
            | Token::Keyword(Keyword::U16)
            | Token::Keyword(Keyword::I16)
            | Token::Keyword(Keyword::U32)
            | Token::Keyword(Keyword::I32)
            | Token::Keyword(Keyword::U64)
            | Token::Keyword(Keyword::I64) => Ok(Self::Integer(parser.parse()?)),
            Token::Symbol(Symbol::And) => Ok(Self::Reference(parser.parse()?)),
            Token::Ident(_) => Ok(Self::Path(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[
                    Expected::Ident,
                    Expected::Symbol(Symbol::And),
                    Expected::Keyword(Keyword::Void),
                    Expected::Keyword(Keyword::Bool),
                    Expected::Keyword(Keyword::U8),
                    Expected::Keyword(Keyword::I8),
                    Expected::Keyword(Keyword::U16),
                    Expected::Keyword(Keyword::I16),
                    Expected::Keyword(Keyword::U32),
                    Expected::Keyword(Keyword::I32),
                    Expected::Keyword(Keyword::U64),
                    Expected::Keyword(Keyword::I64),
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
