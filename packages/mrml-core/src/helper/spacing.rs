use std::convert::TryFrom;

use crate::helper::size::Size;

/// representation of spacing
pub struct Spacing(Size, Option<Size>, Option<Size>, Option<Size>);

impl Spacing {
    pub fn top(&self) -> &Size {
        &self.0
    }

    pub fn right(&self) -> &Size {
        self.1.as_ref().unwrap_or_else(|| self.top())
    }

    pub fn bottom(&self) -> &Size {
        self.2.as_ref().unwrap_or_else(|| self.top())
    }

    pub fn left(&self) -> &Size {
        self.3
            .as_ref()
            .or_else(|| Some(self.right()))
            .unwrap_or_else(|| self.top())
    }
}

impl TryFrom<&str> for Spacing {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut sections = input.split(' ');
        let top = match sections.next() {
            Some(value) => Size::try_from(value)?,
            None => return Err(String::from("no value provided")),
        };
        let right = match sections.next() {
            Some(value) => Some(Size::try_from(value)?),
            None => None,
        };
        let bottom = match sections.next() {
            Some(value) => Some(Size::try_from(value)?),
            None => None,
        };
        let left = match sections.next() {
            Some(value) => Some(Size::try_from(value)?),
            None => None,
        };
        Ok(Spacing(top, right, bottom, left))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::helper::size::Pixel;

    #[test]
    fn single_value() {
        let res: Spacing = Spacing::try_from("1px").unwrap();
        assert_eq!(res.top(), &Size::Pixel(Pixel::new(1.0)));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.top(), res.right());
        assert_eq!(res.right(), res.left());
    }

    #[test]
    fn two_values() {
        let res: Spacing = Spacing::try_from("2px 4px").unwrap();
        assert_eq!(res.top(), &Size::Pixel(Pixel::new(2.0)));
        assert_eq!(res.top(), res.bottom());
        assert_eq!(res.left(), &Size::Pixel(Pixel::new(4.0)));
        assert_eq!(res.left(), res.right());
    }

    #[test]
    fn three_values() {
        let res: Spacing = Spacing::try_from("2px 3px 4px").unwrap();
        assert_eq!(res.top(), &Size::Pixel(Pixel::new(2.0)));
        assert_eq!(res.right(), &Size::Pixel(Pixel::new(3.0)));
        assert_eq!(res.left(), res.right());
        assert_eq!(res.bottom(), &Size::Pixel(Pixel::new(4.0)));
    }

    #[test]
    fn four_values() {
        let res: Spacing = Spacing::try_from("2px 3px 4px 5px").unwrap();
        assert_eq!(res.top(), &Size::Pixel(Pixel::new(2.0)));
        assert_eq!(res.right(), &Size::Pixel(Pixel::new(3.0)));
        assert_eq!(res.bottom(), &Size::Pixel(Pixel::new(4.0)));
        assert_eq!(res.left(), &Size::Pixel(Pixel::new(5.0)));
    }

    #[test]
    fn invalid_values() {
        let res = Spacing::try_from("2tx 3px 4px 5px");
        assert!(res.is_err());
    }
}
