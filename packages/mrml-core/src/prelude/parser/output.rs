pub struct ParseOutput<E> {
    pub element: E,
    pub warnings: Vec<Warning>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarningKind {
    UnexpectedAttribute,
}

impl WarningKind {
    pub const fn as_str(&self) -> &'static str {
        "unexpected-attribute"
    }
}

#[derive(Clone, Debug)]
pub struct Warning {
    pub kind: WarningKind,
    pub span: super::Span,
}

impl<'a> super::MrmlCursor<'a> {
    pub(crate) fn add_warning<S: Into<super::Span>>(&mut self, kind: WarningKind, span: S) {
        self.warnings.push(Warning {
            kind,
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
