use crate::ast::{
    Declaration, FunctionArgument, FunctionDeclaration, Keyword, Punctuated, Symbol, Token,
};
use crate::error::Expected;
use crate::{ast::ReturnType, error::Error};

use super::{Parse, Parser};

impl Parse for ReturnType {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            arrow: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}

impl Parse for FunctionArgument {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            ident: parser.parse()?,
            ty: parser.parse()?,
        })
    }
}

impl Parse for FunctionDeclaration {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let _fn = parser.parse()?;
        let ident = parser.parse()?;
        let open = parser.parse()?;
        let args = Punctuated::parse_terminated(parser, Token::Symbol(Symbol::CloseParen))?;
        let close = parser.parse()?;

        let return_type = if let Token::Symbol(Symbol::Arrow) = parser.peek_token()? {
            Some(parser.parse()?)
        } else {
            None
        };

        Ok(Self {
            _fn,
            ident,
            open,
            args,
            close,
            return_type,
            block: parser.parse()?,
        })
    }
}

impl Parse for Declaration {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Keyword(Keyword::Fn) => Ok(Self::Function(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[Expected::Keyword(Keyword::Fn)],
                tok,
                parser.next_span()?,
            )),
        }
    }
}
