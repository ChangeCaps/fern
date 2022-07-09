use std::fmt::Display;

use crate::ast;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Super,
    Ident(ast::Ident),
}

impl Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathSegment::Super => write!(f, "super"),
            PathSegment::Ident(ident) => write!(f, "{}", ident),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Path {
    pub absolute: Option<ast::ColonColon>,
    pub segments: ast::Punctuated<ast::PathSegment, ast::ColonColon>,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_absolute() {
            write!(f, "::")?;
        }

        let mut segments = self.segments.iter();

        if let Some(segment) = segments.next() {
            write!(f, "{}", segment)?;
        }

        for segment in segments {
            write!(f, "::{}", segment)?;
        }

        Ok(())
    }
}

impl Path {
    pub const fn empty() -> Self {
        Self {
            absolute: None,
            segments: ast::Punctuated::new(),
        }
    }

    pub fn absolute() -> Self {
        Self {
            absolute: Some(ast::ColonColon::default()),
            segments: ast::Punctuated::new(),
        }
    }

    pub const fn is_absolute(&self) -> bool {
        self.absolute.is_some()
    }

    pub fn as_ident(&self) -> Option<&ast::Ident> {
        if self.segments.len() == 1 {
            if let PathSegment::Ident(ref ident) = self.segments[0] {
                Some(ident)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_ident(&self) -> Option<&ast::Ident> {
        if let PathSegment::Ident(ref ident) = self.segments.last()? {
            Some(ident)
        } else {
            None
        }
    }

    pub fn push_ident(&mut self, ident: impl Into<ast::Ident>) {
        self.push_segment(PathSegment::Ident(ident.into()));
    }

    pub fn push_segment(&mut self, segment: PathSegment) {
        self.segments.push(segment, Default::default());
    }

    /// Iterates over the module segments of the path
    pub fn iter_modules(&self) -> impl Iterator<Item = &ast::PathSegment> {
        self.segments.items[0..self.segments.len().saturating_sub(1)].iter()
    }
}

#[macro_export]
macro_rules! path {
    [$($ident:ident)::+] => {{
        let mut path = $crate::ast::Path::empty();
        $(path.push_segment($crate::path!(@segment $ident));)+
        path
    }};
    [::$($ident:ident)::*] => {{
        let mut path = $crate::ast::Path::empty();
        path.absolute = Some(Default::default());
        $(path.push_segment($crate::path!(@segment $ident));)*
        path
    }};
    (@segment $ident:ident) => {
        $crate::ast::PathSegment::Ident($crate::ast::Ident::new(stringify!($ident), $crate::span::Span::null()))
    }
}
