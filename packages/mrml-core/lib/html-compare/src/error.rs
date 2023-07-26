use std::collections::BTreeSet;

use colored::Colorize;
use htmlparser::{StrSpan, Token};

use crate::token::{Attribute, ElementEnd, ElementStart};

#[derive(Debug)]
pub enum ErrorKind<'a> {
    ExpectedElementNotFound {
        expected_parent: StrSpan<'a>,
        expected_element: Token<'a>,
        generated_parent: StrSpan<'a>,
    },
    UnexpectedElementFound {
        generated: Token<'a>,
    },
    ElementMismatch {
        expected: Token<'a>,
        generated: Token<'a>,
    },
    EndOfElementMismatch {
        expected: ElementEnd<'a>,
        generated: ElementEnd<'a>,
    },
    InvalidElementTag {
        expected: ElementStart<'a>,
        generated: ElementStart<'a>,
    },
    ExpectedAttributesNotFound {
        expected: ElementStart<'a>,
        generated: ElementStart<'a>,
        expected_attributes: Vec<Attribute<'a>>,
        generated_attributes: Vec<Attribute<'a>>,
        difference: Vec<StrSpan<'a>>,
    },
    UnexpectedAttributesFound(Vec<StrSpan<'a>>),
    ExpectedAttributeNotFound {
        expected: Attribute<'a>,
    },
    InvalidAttributeValue {
        expected: Attribute<'a>,
        generated: Attribute<'a>,
    },
    ExpectedClassesNotFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    UnexpectedClassesFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    ExpectedStylesNotFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    UnexpectedStylesFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    ExpectedStyleNotFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        missing: &'a str,
    },
    InvalidStyleValue {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        key: &'a str,
        expected_value: &'a str,
        generated_value: &'a str,
    },
    TextMismatch {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
    },
    CssMismatch {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        error: css_compare::Error<'a>,
    },
}

impl<'a> ErrorKind<'a> {
    pub fn display(&self) -> String {
        // TODO improve error display
        format!("{self:?}")
    }
}

#[derive(Debug)]
pub struct Error<'a> {
    pub expected: &'a str,
    pub generated: &'a str,
    pub kind: ErrorKind<'a>,
}

fn display_subset(data: &str, span_start: usize, span_end: usize, gap: usize) -> String {
    let start = span_start.saturating_sub(gap);
    let end = usize::min(span_end + gap, data.len());
    format!(
        "{}{}{}",
        &data[start..span_start],
        data[span_start..span_end].red().bold(),
        &data[span_end..end]
    )
}

const SUBSET_GAP: usize = 150;

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::ElementMismatch {
                expected,
                generated,
            } => {
                writeln!(f, "= Element mismatch")?;
                writeln!(f, "== Expected result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(
                        self.expected,
                        expected.span().start(),
                        expected.span().end(),
                        SUBSET_GAP
                    )
                )?;
                writeln!(f)?;
                writeln!(f, "== Generated result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(
                        self.generated,
                        generated.span().start(),
                        generated.span().end(),
                        SUBSET_GAP
                    )
                )?;
                writeln!(f)?;
            }
            ErrorKind::TextMismatch {
                expected,
                generated,
            } => {
                writeln!(f, "= Text mismatch")?;
                writeln!(f, "== Expected result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(self.expected, expected.start(), expected.end(), SUBSET_GAP)
                )?;
                writeln!(f)?;
                writeln!(f, "== Generated result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(
                        self.generated,
                        generated.start(),
                        generated.end(),
                        SUBSET_GAP
                    )
                )?;
                writeln!(f)?;
            }
            ErrorKind::ExpectedStylesNotFound {
                expected,
                generated,
                difference,
            } => {
                writeln!(f, "= Expected styles not found")?;
                writeln!(f, "== Expected result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(self.expected, expected.start(), expected.end(), SUBSET_GAP)
                )?;
                writeln!(f)?;
                writeln!(f, "== Generated result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(
                        self.generated,
                        generated.start(),
                        generated.end(),
                        SUBSET_GAP
                    )
                )?;
                writeln!(f)?;
                writeln!(f, "== Problem")?;
                writeln!(f, "Missing {difference:?}")?;
            }
            ErrorKind::ExpectedAttributesNotFound {
                expected,
                generated,
                expected_attributes,
                generated_attributes,
                difference,
            } => {
                writeln!(f, "= Expected attributes not found")?;
                writeln!(f, "== Expected result")?;
                let span_start = expected.span.end();
                let span_end = expected_attributes
                    .iter()
                    .map(|attr| attr.value.end())
                    .max()
                    .unwrap_or(span_start);
                writeln!(
                    f,
                    "{}",
                    display_subset(self.expected, span_start, span_end, SUBSET_GAP)
                )?;
                writeln!(f)?;
                writeln!(f, "== Generated result")?;
                let span_start = generated.span.end();
                let span_end = generated_attributes
                    .iter()
                    .map(|attr| attr.value.end())
                    .max()
                    .unwrap_or(span_start);
                writeln!(
                    f,
                    "{}",
                    display_subset(self.generated, span_start, span_end, SUBSET_GAP)
                )?;
                writeln!(f)?;
                writeln!(f, "== Problem")?;
                writeln!(f, "Missing {difference:?}")?;
            }
            ErrorKind::CssMismatch {
                expected,
                generated,
                error,
            } => {
                writeln!(f, "= CSS mismatch")?;
                writeln!(f, "== Expected result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(self.expected, expected.start(), expected.end(), SUBSET_GAP)
                )?;
                writeln!(f)?;
                writeln!(f, "== Generated result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(
                        self.generated,
                        generated.start(),
                        generated.end(),
                        SUBSET_GAP
                    )
                )?;
                writeln!(f)?;
                writeln!(f, "== Problem")?;
                writeln!(f, "Missing {error:?}")?;
            }
            _ => {
                writeln!(f, "{:?}", self.kind)?;
            }
        }
        Ok(())
    }
}
