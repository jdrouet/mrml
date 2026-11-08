use std::str::FromStr;

use crate::helper::size::{Pixel, SizeParserError};

#[derive(Debug, thiserror::Error)]
pub enum SpacingParserError {
    #[error("{0}")]
    InvalidSize(
        #[from]
        #[source]
        SizeParserError,
    ),
    #[error("no value provided")]
    Empty,
}

/// representation of spacing
pub struct Spacing([Pixel; 4]);

impl Spacing {
    pub fn top(&self) -> Pixel {
        self.0[0]
    }

    pub fn right(&self) -> Pixel {
        self.0[1]
    }

    pub fn bottom(&self) -> Pixel {
        self.0[2]
    }

    pub fn left(&self) -> Pixel {
        self.0[3]
    }
}

impl FromStr for Spacing {
    type Err = SpacingParserError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut sections = input.split(' ');
        match (
            sections.next(),
            sections.next(),
            sections.next(),
            sections.next(),
        ) {
            (Some(first), None, None, None) => {
                let first = Pixel::from_str(first)?;
                Ok(Self([first, first, first, first]))
            }
            (Some(first), Some(second), None, None) => {
                let first = Pixel::from_str(first)?;
                let second = Pixel::from_str(second)?;
                Ok(Self([first, second, first, second]))
            }
            (Some(first), Some(second), Some(third), None) => {
                let first = Pixel::from_str(first)?;
                let second = Pixel::from_str(second)?;
                let third = Pixel::from_str(third)?;
                Ok(Self([first, second, third, second]))
            }
            (Some(first), Some(second), Some(third), Some(four)) => Ok(Self([
                Pixel::from_str(first)?,
                Pixel::from_str(second)?,
                Pixel::from_str(third)?,
                Pixel::from_str(four)?,
            ])),
            _ => Err(SpacingParserError::Empty),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::helper::size::Pixel;

    #[test]
    fn single_value() {
        let res: Spacing = Spacing::from_str("1px").unwrap();
        assert_eq!(res.top(), Pixel::new(1.0));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.top(), res.right());
        assert_eq!(res.right(), res.left());
    }

    #[test]
    fn two_values() {
        let res: Spacing = Spacing::from_str("2px 4px").unwrap();
        assert_eq!(res.top(), Pixel::new(2.0));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.left(), Pixel::new(4.0));
        assert_eq!(res.left(), res.right());
    }

    #[test]
    fn three_values() {
        let res: Spacing = Spacing::from_str("2px 3px 4px").unwrap();
        assert_eq!(res.top(), Pixel::new(2.0));
        assert_eq!(res.right(), Pixel::new(3.0));
        assert_eq!(res.left(), res.right());
        assert_eq!(res.bottom(), Pixel::new(4.0));
    }

    #[test]
    fn four_values() {
        let res: Spacing = Spacing::from_str("2px 3px 4px 5px").unwrap();
        assert_eq!(res.top(), Pixel::new(2.0));
        assert_eq!(res.right(), Pixel::new(3.0));
        assert_eq!(res.bottom(), Pixel::new(4.0));
        assert_eq!(res.left(), Pixel::new(5.0));
    }

    #[test]
    fn invalid_values() {
        let res = Spacing::from_str("2tx 3px 4px 5px");
        assert!(res.is_err());
    }

    #[test]
    fn unitless_zero() {
        let res: Spacing = Spacing::from_str("20px 20px 0 20px").unwrap();
        assert_eq!(res.top(), Pixel::new(20.0));
        assert_eq!(res.right(), Pixel::new(20.0));
        assert_eq!(res.bottom(), Pixel::new(0.0));
        assert_eq!(res.left(), Pixel::new(20.0));
    }

    // pin deliberate parser quirks
    #[test]
    fn tab_is_not_a_separator() {
        let res = Spacing::from_str("2px\t4px");
        assert!(res.is_err());
    }

    #[test]
    fn double_space_fails_to_parse() {
        let res = Spacing::from_str("2px  4px");
        assert!(res.is_err());
    }

    #[test]
    fn fifth_value_is_ignored() {
        let res: Spacing = Spacing::from_str("1px 2px 3px 4px 5px").unwrap();
        assert_eq!(res.top(), Pixel::new(1.0));
        assert_eq!(res.right(), Pixel::new(2.0));
        assert_eq!(res.bottom(), Pixel::new(3.0));
        assert_eq!(res.left(), Pixel::new(4.0));
    }
}
