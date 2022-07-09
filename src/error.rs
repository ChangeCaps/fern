use std::{fmt::Display, panic::Location};

use crate::{
    ast::{Keyword, Symbol},
    span::Span,
};

#[derive(Clone, Debug)]
pub struct Error {
    message: String,
    span: Option<Span>,
    hints: Vec<ErrorHint>,
    location: &'static Location<'static>,
}

impl Error {
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
            hints: Vec::new(),
            location: Location::caller(),
        }
    }

    #[track_caller]
    pub fn spanned(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
            hints: Vec::new(),
            location: Location::caller(),
        }
    }

    pub fn add_hint(&mut self, hint: ErrorHint) {
        self.hints.push(hint);
    }

    pub fn with_hint(mut self, message: impl Into<String>, span: Span) -> Self {
        self.add_hint(ErrorHint::new(message, span));
        self
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn span(&self) -> Option<Span> {
        self.span
    }

    pub fn hints(&self) -> &[ErrorHint] {
        &self.hints
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expected {
    Ident,
    String,
    Integer,
    Symbol(Symbol),
    Keyword(Keyword),
    Expression,
}

impl Display for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident => write!(f, "identifier"),
            Self::String => write!(f, "string literal"),
            Self::Integer => write!(f, "integer"),
            Self::Symbol(symbol) => write!(f, "'{}'", symbol),
            Self::Keyword(keyword) => write!(f, "'{}'", keyword),
            Self::Expression => write!(f, "expression"),
        }
    }
}

impl Error {
    #[track_caller]
    pub fn unexpected_eof(span: Span) -> Self {
        Self::spanned("Unexpected end of file", span)
    }

    #[track_caller]
    pub fn expected(expected: Expected, found: impl Display, span: Span) -> Self {
        Self::spanned(format!("Expected {} found {}", expected, found), span)
    }

    #[track_caller]
    pub fn expected_any(expected: &[Expected], found: impl Display, span: Span) -> Self {
        Self::spanned(
            format!("Expected any of {:?} found {}", expected, found),
            span,
        )
    }
}

#[derive(Clone, Debug)]
pub struct ErrorHint {
    message: String,
    span: Span,
}

impl ErrorHint {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
