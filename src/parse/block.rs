use crate::{
    ast::{Block, OpenBrace, Symbol, Token},
    error::Error,
};

use super::{Parse, Parser};

impl Parse for Block {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let open = parser.parse::<OpenBrace>()?;

        let mut statements = Vec::new();

        loop {
            if let Token::Symbol(Symbol::CloseBrace) = parser.peek_token()? {
                break;
            } else {
                statements.push(parser.parse()?);
            }
        }

        Ok(Self {
            open,
            statements,
            close: parser.parse()?,
        })
    }
}
