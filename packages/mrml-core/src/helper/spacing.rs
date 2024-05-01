use std::convert::TryFrom;

use crate::helper::size::Pixel;

/// representation of spacing
pub struct Spacing {
    top: Pixel,
    right: Option<Pixel>,
    bottom: Option<Pixel>,
    left: Option<Pixel>,
}

impl Spacing {
    pub fn top(&self) -> &Pixel {
        &self.top
    }

    pub fn into_top(self) -> Pixel {
        self.top
    }

    pub fn right(&self) -> &Pixel {
        self.right.as_ref().unwrap_or_else(|| self.top())
    }

    pub fn into_right(self) -> Pixel {
        if let Some(v) = self.right {
            v
        } else {
            self.into_top()
        }
    }

    pub fn bottom(&self) -> &Pixel {
        self.bottom.as_ref().unwrap_or_else(|| self.top())
    }

    pub fn into_bottom(self) -> Pixel {
        if let Some(v) = self.bottom {
            v
        } else {
            self.into_top()
        }
    }

    pub fn left(&self) -> &Pixel {
        self.left
            .as_ref()
            .or_else(|| Some(self.right()))
            .unwrap_or_else(|| self.top())
    }

    pub fn into_left(self) -> Pixel {
        if let Some(v) = self.left {
            v
        } else {
            self.into_right()
        }
    }
}

impl TryFrom<&str> for Spacing {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut sections = input.split(' ');
        let top = match sections.next() {
            Some(value) => Pixel::try_from(value)?,
            None => return Err(String::from("no value provided")),
        };
        let right = match sections.next() {
            Some(value) => Some(Pixel::try_from(value)?),
            None => None,
        };
        let bottom = match sections.next() {
            Some(value) => Some(Pixel::try_from(value)?),
            None => None,
        };
        let left = match sections.next() {
            Some(value) => Some(Pixel::try_from(value)?),
            None => None,
        };
        Ok(Spacing {
            top,
            right,
            bottom,
            left,
        })
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
