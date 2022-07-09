use crate::{
    ast::{Program, Token},
    error::Error,
};

use super::{Parse, Parser};

impl Parse for Program {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let mut declarations = Vec::new();

        loop {
            if let Token::Eof = parser.peek_token()? {
                break;
            } else {
                declarations.push(parser.parse()?);
            }
        }

        Ok(Self { declarations })
    }
}
