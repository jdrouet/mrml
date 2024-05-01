use std::convert::TryFrom;

use crate::helper::size::Pixel;

/// representation of spacing
pub enum Spacing {
    Single(Pixel),
    Two(Pixel, Pixel),
    Three(Pixel, Pixel, Pixel),
    Four(Pixel, Pixel, Pixel, Pixel),
}

impl Spacing {
    pub fn top(&self) -> &Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(vertical, _horizontal) => vertical,
            Self::Three(top, _horizontal, _bottom) => top,
            Self::Four(top, _right, _bottom, _left) => top,
        }
    }

    pub fn into_top(self) -> Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(vertical, _horizontal) => vertical,
            Self::Three(top, _horizontal, _bottom) => top,
            Self::Four(top, _right, _bottom, _left) => top,
        }
    }

    pub fn right(&self) -> &Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(_vertical, horizontal) => horizontal,
            Self::Three(_top, horizontal, _bottom) => horizontal,
            Self::Four(_top, right, _bottom, _left) => right,
        }
    }

    pub fn into_right(self) -> Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(_vertical, horizontal) => horizontal,
            Self::Three(_top, horizontal, _bottom) => horizontal,
            Self::Four(_top, right, _bottom, _left) => right,
        }
    }

    pub fn bottom(&self) -> &Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(vertical, _horizontal) => vertical,
            Self::Three(_top, _horizontal, bottom) => bottom,
            Self::Four(_top, _right, bottom, _left) => bottom,
        }
    }

    pub fn into_bottom(self) -> Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(vertical, _horizontal) => vertical,
            Self::Three(_top, _horizontal, bottom) => bottom,
            Self::Four(_top, _right, bottom, _left) => bottom,
        }
    }

    pub fn left(&self) -> &Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(_vertical, horizontal) => horizontal,
            Self::Three(_top, horizontal, _bottom) => horizontal,
            Self::Four(_top, _right, _bottom, left) => left,
        }
    }

    pub fn into_left(self) -> Pixel {
        match self {
            Self::Single(top) => top,
            Self::Two(_vertical, horizontal) => horizontal,
            Self::Three(_top, horizontal, _bottom) => horizontal,
            Self::Four(_top, _right, _bottom, left) => left,
        }
    }
}

impl std::fmt::Display for Spacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(first) => write!(f, "{first}"),
            Self::Two(first, second) => write!(f, "{first} {second}"),
            Self::Three(first, second, third) => write!(f, "{first} {second} {third}"),
            Self::Four(first, second, third, fourth) => {
                write!(f, "{first} {second} {third} {fourth}")
            }
        }
    }
}

impl TryFrom<&str> for Spacing {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut sections = input.split(' ');
        match (
            sections.next(),
            sections.next(),
            sections.next(),
            sections.next(),
        ) {
            (Some(first), None, None, None) => Ok(Self::Single(Pixel::try_from(first)?)),
            (Some(first), Some(second), None, None) => {
                Ok(Self::Two(Pixel::try_from(first)?, Pixel::try_from(second)?))
            }
            (Some(first), Some(second), Some(third), None) => Ok(Self::Three(
                Pixel::try_from(first)?,
                Pixel::try_from(second)?,
                Pixel::try_from(third)?,
            )),
            (Some(first), Some(second), Some(third), Some(four)) => Ok(Self::Four(
                Pixel::try_from(first)?,
                Pixel::try_from(second)?,
                Pixel::try_from(third)?,
                Pixel::try_from(four)?,
            )),
            _ => Err(String::from("no value provided")),
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
        assert_eq!(res.top(), &Pixel::new(1.0));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.top(), res.right());
        assert_eq!(res.right(), res.left());
    }

    #[test]
    fn two_values() {
        let res: Spacing = Spacing::try_from("2px 4px").unwrap();
        assert_eq!(res.top(), &Pixel::new(2.0));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.left(), &Pixel::new(4.0));
        assert_eq!(res.left(), res.right());
    }

    #[test]
    fn three_values() {
        let res: Spacing = Spacing::try_from("2px 3px 4px").unwrap();
        assert_eq!(res.top(), &Pixel::new(2.0));
        assert_eq!(res.right(), &Pixel::new(3.0));
        assert_eq!(res.left(), res.right());
        assert_eq!(res.bottom(), &Pixel::new(4.0));
    }

    #[test]
    fn four_values() {
        let res: Spacing = Spacing::try_from("2px 3px 4px 5px").unwrap();
        assert_eq!(res.top(), &Pixel::new(2.0));
        assert_eq!(res.right(), &Pixel::new(3.0));
        assert_eq!(res.bottom(), &Pixel::new(4.0));
        assert_eq!(res.left(), &Pixel::new(5.0));
    }

    #[test]
    fn invalid_values() {
        let res = Spacing::try_from("2tx 3px 4px 5px");
        assert!(res.is_err());
    }
}
