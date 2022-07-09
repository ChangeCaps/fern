use crate::{
    ast::{Path, PathSegment, Punctuated, Symbol, Token},
    error::{Error, Expected},
};

use super::{Parse, Parser};

impl Parse for PathSegment {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        match parser.peek_token()? {
            Token::Ident(_) => Ok(Self::Ident(parser.parse()?)),
            tok => Err(Error::expected_any(
                &[Expected::Ident],
                tok,
                parser.next_span()?,
            )),
        }
    }
}

impl Parse for Path {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let absolute = if let Token::Symbol(Symbol::ColonColon) = parser.peek_token()? {
            Some(parser.parse()?)
        } else {
            None
        };

        let mut segments = Punctuated::new();

        loop {
            segments.items.push(parser.parse()?);

            if parser.peek_token()? != Token::Symbol(Symbol::ColonColon) {
                break;
            }

            segments.punct.push(parser.parse()?);
        }

        Ok(Self { absolute, segments })
    }
}
