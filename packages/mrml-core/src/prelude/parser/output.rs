pub struct ParseOutput<E> {
    pub element: E,
    pub warnings: Vec<Warning>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarningKind {
    UnexpectedAttribute,
    UnknownElement,
}

impl WarningKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::UnexpectedAttribute => "unexpected-attribute",
            Self::UnknownElement => "unknown-element",
        }
    }
}

impl std::fmt::Display for WarningKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedAttribute => f.write_str("unexpected attribute"),
            Self::UnknownElement => f.write_str("unknown element"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Warning {
    pub kind: WarningKind,
    pub origin: super::Origin,
    pub span: super::Span,
}

impl super::MrmlCursor<'_> {
    pub(crate) fn add_warning<S: Into<super::Span>>(&mut self, kind: WarningKind, span: S) {
        self.warnings.push(Warning {
            kind,
            origin: self.origin.clone(),
            span: span.into(),
        });
    }

    pub(crate) fn warnings(self) -> Vec<Warning> {
        self.warnings
    }

    pub(crate) fn with_warnings(&mut self, others: Vec<Warning>) {
        self.warnings.extend(others);
    }
}

impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} in {} at position {}",
            self.kind, self.origin, self.span
        )
    }
}
