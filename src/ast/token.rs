use std::{fmt::Display, hash::Hash};

use crate::{
    error::{Error, Expected},
    span::{Span, Spanned},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(String),
    String(String),
    Integer(Integer),
    Keyword(Keyword),
    Symbol(Symbol),
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident) => write!(f, "'{}'", ident),
            Self::String(string) => write!(f, "\"{}\"", string),
            Self::Integer(integer) => write!(f, "'{}'", integer),
            Self::Keyword(keyword) => write!(f, "'{}'", keyword),
            Self::Symbol(symbol) => write!(f, "'{}'", symbol),
            Self::Eof => write!(f, "end of file"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntegerKind {
    Binary,
    Decimal,
    Hex,
}

impl IntegerKind {
    pub const fn radix(&self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Decimal => 10,
            Self::Hex => 16,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Integer {
    value: i64,
    kind: IntegerKind,
}

impl Integer {
    pub const fn new(value: i64, kind: IntegerKind) -> Self {
        Self { value, kind }
    }

    pub const fn value(&self) -> i64 {
        self.value
    }

    pub const fn kind(&self) -> IntegerKind {
        self.kind
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            IntegerKind::Binary => write!(f, "0b{:b}", self.value),
            IntegerKind::Decimal => write!(f, "{}", self.value),
            IntegerKind::Hex => write!(f, "0x{:x}", self.value),
        }
    }
}

#[derive(Clone)]
pub struct Ident {
    string: String,
    span: Span,
}

impl Ident {
    pub fn new(string: impl Into<String>, span: Span) -> Self {
        Self {
            string: string.into(),
            span,
        }
    }

    pub fn string(&self) -> &String {
        &self.string
    }
}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident({})", self.string)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

#[cfg(feature = "parse")]
impl crate::parse::Parse for Ident {
    fn parse(parser: &mut crate::parse::Parser) -> Result<Self, Error> {
        let span = parser.next_span()?;

        match parser.next_token()? {
            Token::Ident(string) => Ok(Self { string, span }),
            tok => Err(Error::expected(Expected::String, tok, span)),
        }
    }
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.string.eq(&other.string)
    }
}

impl Eq for Ident {}

impl Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.string.hash(state);
    }
}

#[derive(Clone)]
pub struct IntegerLiteral {
    integer: Integer,
    span: Span,
}

impl IntegerLiteral {
    pub fn integer(&self) -> &Integer {
        &self.integer
    }
}

impl std::fmt::Debug for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.integer)
    }
}

#[cfg(feature = "parse")]
impl crate::parse::Parse for IntegerLiteral {
    fn parse(parser: &mut crate::parse::Parser) -> Result<Self, Error> {
        let span = parser.next_span()?;

        match parser.next_token()? {
            Token::Integer(integer) => Ok(Self { integer, span }),
            tok => Err(Error::expected(Expected::String, tok, span)),
        }
    }
}

impl Spanned for IntegerLiteral {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone)]
pub struct StringLiteral {
    string: String,
    span: Span,
}

impl StringLiteral {
    pub fn string(&self) -> &String {
        &self.string
    }
}

impl std::fmt::Debug for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.string)
    }
}

#[cfg(feature = "parse")]
impl crate::parse::Parse for StringLiteral {
    fn parse(parser: &mut crate::parse::Parser) -> Result<Self, Error> {
        let span = parser.next_span()?;

        match parser.next_token()? {
            Token::String(string) => Ok(Self { string, span }),
            tok => Err(Error::expected(Expected::String, tok, span)),
        }
    }
}

impl Spanned for StringLiteral {
    fn span(&self) -> Span {
        self.span
    }
}

macro_rules! symbols {
    ($($first:literal $(, $second:literal)? => $ident:ident),* $(,)?) => {
        $(
            #[derive(Clone, Copy)]
            pub struct $ident(Span);

            impl Default for $ident {
                fn default() -> Self {
                    Self(Span::null())
                }
            }

            impl Spanned for $ident {
                fn span(&self) -> Span {
                    self.0
                }
            }

            impl std::fmt::Debug for $ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, stringify!($ident))
                }
            }

            #[cfg(feature = "parse")]
            impl $crate::parse::Parse for $ident {
                fn parse(parser: &mut $crate::parse::Parser) -> Result<Self, Error> {
                    let span = parser.span();

                    match parser.next_token()? {
                        Token::Symbol(Symbol::$ident) => {
                            let span = span | parser.span();

                            Ok(Self(span))
                        },
                        tok => {
                            let span = span | parser.span();

                            #[allow(unused_mut)]
                            let mut symbol = String::from($first);
                            $(symbol.push($second);)?

                            Err(Error::spanned(format!("Expected symbol '{}' found '{}'", symbol, tok), span))
                        },
                    }
                }
            }

            impl PartialEq for $ident {
                fn eq(&self, _: &Self) -> bool {
                    true
                }
            }

            impl Eq for $ident {}

            impl Hash for $ident {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    state.write_u8(0);
                }
            }
        )*

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Symbol {
            $($ident),*
        }

        impl Display for Symbol {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$ident => {
                            #[allow(unused_mut)]
                            let mut symbol = String::from($first);
                            $(symbol.push($second);)?

                            write!(f, "{}", symbol)
                        }
                    )*
                }
            }
        }

        pub(crate) fn symbol_from_chars(first: char, second: Option<char>) -> Option<(Symbol, bool)> {
            match first {
                $(
                    $first $(if second == Some($second))? => Some((
                        Symbol::$ident,
                        false $(|| { let _ = $second; true })?
                    )),
                )*
                _ => None,
            }
        }
    };
}

macro_rules! keywords {
    ($($keyword:literal => $ident:ident),* $(,)?) => {
        $(
            #[derive(Clone, Copy)]
            pub struct $ident(Span);

            impl Default for $ident {
                fn default() -> Self {
                    Self(Span::null())
                }
            }

            impl Spanned for $ident {
                fn span(&self) -> Span {
                    self.0
                }
            }

            impl std::fmt::Debug for $ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, stringify!($ident))
                }
            }

            #[cfg(feature = "parse")]
            impl $crate::parse::Parse for $ident {
                fn parse(parser: &mut $crate::parse::Parser) -> Result<Self, Error> {
                    let span = parser.span();

                    match parser.next_token()? {
                        Token::Keyword(Keyword::$ident) => {
                            let span = span | parser.span();

                            Ok(Self(span))
                        },
                        tok => {
                            let span = span | parser.span();

                            Err(Error::spanned(format!("Expected keyword '{}' found '{}'", $keyword, tok), span))
                        },
                    }
                }
            }

            impl PartialEq for $ident {
                fn eq(&self, _: &Self) -> bool {
                    true
                }
            }

            impl Eq for $ident {}

            impl Hash for $ident {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    state.write_u8(0);
                }
            }
        )*

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Keyword {
            $($ident),*
        }

        impl Display for Keyword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$ident => {
                            write!(f, $keyword)
                        }
                    )*
                }
            }
        }

        pub(crate) fn keyword_from_string(string: &str) -> Option<Keyword> {
            match string {
                $($keyword => Some(Keyword::$ident),)*
                _ => None,
            }
        }
    };
}

symbols! {
    '>', '>' => ShiftRight,
    '<', '<' => ShiftLeft,
    '-', '>' => Arrow,
    '=', '>' => FatArrow,
    '&', '&' => AndAnd,
    '|', '|' => OrOr,
    ':', ':' => ColonColon,
    '(' => OpenParen,
    ')' => CloseParen,
    '[' => OpenBracket,
    ']' => CloseBracket,
    '{' => OpenBrace,
    '}' => CloseBrace,
    '+' => Plus,
    '-' => Minus,
    '*' => Asterisk,
    '/' => Slash,
    '&' => And,
    '|' => Or,
    '=' => Equal,
    '.' => Dot,
    ',' => Comma,
    ':' => Colon,
    ';' => SemiColon,
}

keywords! {
    "fn" => Fn,
    "let" => Let,
    "return" => Return,
    "void" => Void,
    "bool" => Bool,
    "true" => True,
    "false" => False,
    "i8" => I8,
    "u8" => U8,
    "i16" => I16,
    "u16" => U16,
    "i32" => I32,
    "u32" => U32,
    "i64" => I64,
    "u64" => U64,
}
