use std::convert::TryFrom;

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

impl TryFrom<&str> for Spacing {
    type Error = SpacingParserError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut sections = input.split(' ');
        match (
            sections.next(),
            sections.next(),
            sections.next(),
            sections.next(),
        ) {
            (Some(first), None, None, None) => {
                let first = Pixel::try_from(first)?;
                Ok(Self([first, first, first, first]))
            }
            (Some(first), Some(second), None, None) => {
                let first = Pixel::try_from(first)?;
                let second = Pixel::try_from(second)?;
                Ok(Self([first, second, first, second]))
            }
            (Some(first), Some(second), Some(third), None) => {
                let first = Pixel::try_from(first)?;
                let second = Pixel::try_from(second)?;
                let third = Pixel::try_from(third)?;
                Ok(Self([first, second, third, second]))
            }
            (Some(first), Some(second), Some(third), Some(four)) => Ok(Self([
                Pixel::try_from(first)?,
                Pixel::try_from(second)?,
                Pixel::try_from(third)?,
                Pixel::try_from(four)?,
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
        let res: Spacing = Spacing::try_from("1px").unwrap();
        assert_eq!(res.top(), Pixel::new(1.0));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.top(), res.right());
        assert_eq!(res.right(), res.left());
    }

    #[test]
    fn two_values() {
        let res: Spacing = Spacing::try_from("2px 4px").unwrap();
        assert_eq!(res.top(), Pixel::new(2.0));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.left(), Pixel::new(4.0));
        assert_eq!(res.left(), res.right());
    }

    #[test]
    fn three_values() {
        let res: Spacing = Spacing::try_from("2px 3px 4px").unwrap();
        assert_eq!(res.top(), Pixel::new(2.0));
        assert_eq!(res.right(), Pixel::new(3.0));
        assert_eq!(res.left(), res.right());
        assert_eq!(res.bottom(), Pixel::new(4.0));
    }

    #[test]
    fn four_values() {
        let res: Spacing = Spacing::try_from("2px 3px 4px 5px").unwrap();
        assert_eq!(res.top(), Pixel::new(2.0));
        assert_eq!(res.right(), Pixel::new(3.0));
        assert_eq!(res.bottom(), Pixel::new(4.0));
        assert_eq!(res.left(), Pixel::new(5.0));
    }

    #[test]
    fn invalid_values() {
        let res = Spacing::try_from("2tx 3px 4px 5px");
        assert!(res.is_err());
    }

    #[test]
    fn unitless_zero() {
        let res: Spacing = Spacing::try_from("20px 20px 0 20px").unwrap();
        assert_eq!(res.top(), Pixel::new(20.0));
        assert_eq!(res.right(), Pixel::new(20.0));
        assert_eq!(res.bottom(), Pixel::new(0.0));
        assert_eq!(res.left(), Pixel::new(20.0));
    }
}
