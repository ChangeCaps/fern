use crate::{
    ast::{Punctuated, Token},
    error::Error,
};

use super::{Parse, Parser};

impl<T, U> Punctuated<T, U>
where
    T: Parse,
    U: Parse,
{
    pub fn parse(parser: &mut Parser, termination: Token) -> Result<Self, Error> {
        let mut this = Self::new();

        if parser.peek_token()? != termination {
            loop {
                this.items.push(parser.parse()?);

                if parser.peek_token()? == termination {
                    break;
                }

                this.punct.push(parser.parse()?);
            }
        }

        Ok(this)
    }

    pub fn parse_terminated(parser: &mut Parser, termination: Token) -> Result<Self, Error> {
        let mut this = Self::new();

        loop {
            if parser.peek_token()? == termination {
                break;
            }

            this.items.push(parser.parse()?);

            if parser.peek_token()? == termination {
                break;
            }

            this.punct.push(parser.parse()?);
        }

        Ok(this)
    }
}
