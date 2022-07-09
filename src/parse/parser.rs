use std::{hint::unreachable_unchecked, iter::Peekable, str::Chars};

use crate::{
    ast::{keyword_from_string, symbol_from_chars, Integer, IntegerKind, Token},
    error::Error,
    source::SourceId,
    span::Span,
};

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
    peeked_token: Option<(Token, Span)>,
    source: SourceId,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, source_id: SourceId) -> Self {
        Self {
            chars: source.chars().peekable(),
            peeked_token: None,
            source: source_id,
            index: 0,
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.skip_whitespace();
        self.chars.peek().is_none()
    }

    pub fn span(&self) -> Span {
        Span::new(self.source, self.index, 0)
    }

    pub fn char_span(&self) -> Span {
        Span::new(self.source, self.index, 1)
    }

    pub fn next_char(&mut self) -> Result<char, Error> {
        if let Some(ch) = self.chars.next() {
            self.index += ch.len_utf8();

            Ok(ch)
        } else {
            Err(Error::unexpected_eof(self.span()))
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn skip_char(&mut self) {
        let _ = self.next_char();
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(ch) if ch.is_whitespace() => self.skip_char(),
                _ => break,
            }
        }
    }

    /// [`self`] *must* not be [`Self::empty`].
    fn parse_ident(&mut self) -> Result<String, Error> {
        let mut ident = String::new();

        loop {
            if let Some(ch) = self.peek_char() {
                if ch == '_' || ch.is_alphabetic() || ch.is_numeric() {
                    self.skip_char();

                    ident.push(ch);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(ident)
    }

    fn parse_string(&mut self) -> Result<String, Error> {
        self.skip_char();
        let mut string = String::new();

        loop {
            let ch = self.next_char()?;

            if ch == '"' {
                break;
            } else {
                string.push(ch);
            }
        }

        Ok(string)
    }

    fn parse_number(&mut self) -> Result<Token, Error> {
        let span = self.span();
        let mut value: i64 = 0;
        let mut digits = 0;
        let mut kind = IntegerKind::Decimal;

        loop {
            if let Some(ch) = self.peek_char() {
                if value == 0 && digits == 1 && kind == IntegerKind::Decimal && ch == 'b' {
                    self.skip_char();
                    digits = 0;
                    kind = IntegerKind::Binary;
                } else if value == 0 && digits == 1 && kind == IntegerKind::Decimal && ch == 'x' {
                    self.skip_char();
                    digits = 0;
                    kind = IntegerKind::Hex;
                } else if let Some(digit) = ch.to_digit(kind.radix()) {
                    self.skip_char();
                    value *= kind.radix() as i64;
                    value += digit as i64;
                    digits += 1;
                } else if ch.is_digit(10) {
                    let span = span | self.char_span();
                    return Err(Error::spanned("Integer contains invalid digit", span)
                        .with_hint("Here", self.char_span()));
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if digits == 0 {
            return Err(Error::spanned(
                "Integers must contain at least one digit",
                span | self.span(),
            ));
        }

        Ok(Token::Integer(Integer::new(value, kind)))
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        if let Some((token, _)) = self.peeked_token.take() {
            return Ok(token);
        }

        if self.is_empty() {
            return Ok(Token::Eof);
        }

        let span = self.span();

        match self.peek_char() {
            Some(ch) if ch == '_' || ch.is_alphabetic() => {
                let ident = self.parse_ident()?;

                if let Some(keyword) = keyword_from_string(&ident) {
                    Ok(Token::Keyword(keyword))
                } else {
                    Ok(Token::Ident(ident))
                }
            }
            Some(ch) if ch.is_digit(10) => self.parse_number(),
            Some(ch) if ch == '"' => {
                let string = self.parse_string()?;

                Ok(Token::String(string))
            }
            Some(ch) => {
                self.skip_char();

                if let Some((symbol, should_skip)) = symbol_from_chars(ch, self.peek_char()) {
                    if should_skip {
                        self.skip_char();
                    }

                    Ok(Token::Symbol(symbol))
                } else {
                    let span = span | self.span();
                    Err(Error::spanned(format!("Unexpected symbol '{}'", ch), span))
                }
            }
            None => {
                // SAFETY: if there are no more characters, we would have returned Token::Eof earlier
                unsafe { unreachable_unchecked() }
            }
        }
    }

    fn peek_next_token(&mut self) -> Result<(), Error> {
        self.skip_whitespace();
        let mut span = self.span();
        let token = self.next_token()?;
        span |= self.span();

        self.peeked_token = Some((token, span));

        Ok(())
    }

    pub fn peek_token(&mut self) -> Result<Token, Error> {
        if let Some((ref token, _)) = self.peeked_token {
            Ok(token.clone())
        } else {
            self.peek_next_token()?;

            // SAFETY: self::peeked_token was just set to Some
            Ok(unsafe { self.peeked_token.as_ref().unwrap_unchecked().0.clone() })
        }
    }

    pub fn next_span(&mut self) -> Result<Span, Error> {
        if self.peeked_token.is_none() {
            self.peek_next_token()?;
        }

        Ok(unsafe { self.peeked_token.as_ref().unwrap_unchecked().1 })
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, Error> {
        T::parse(self)
    }
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, Error>;
}

impl<T: Parse> Parse for Box<T> {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Box::new(parser.parse()?))
    }
}
