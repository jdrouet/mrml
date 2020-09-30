use super::size::{ParseSizeError, Size};
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseSpacingError {
    NoValueProvided,
    TooManyValuesProvided,
}

#[derive(Debug)]
pub enum Error {
    SizeError(ParseSizeError),
    SpacingError(ParseSpacingError),
}

impl From<ParseSizeError> for Error {
    fn from(error: ParseSizeError) -> Self {
        Error::SizeError(error)
    }
}

/// representation of spacing
///
/// ```rust
/// use mrml::util::size::Size;
/// use mrml::util::spacing::Spacing;
/// use std::str::FromStr;
/// let res = Spacing::from_str("1px 2px 3% 4px");
/// assert!(res.is_ok(), true);
/// let spacing = res.unwrap();
/// assert_eq!(spacing.top, Size::Pixel(1.0));
/// assert_eq!(spacing.bottom, Size::Percent(3.0));
/// ```
#[derive(Clone, Debug)]
pub struct Spacing {
    pub top: Size,
    pub right: Size,
    pub bottom: Size,
    pub left: Size,
}

impl Spacing {
    pub fn from_4d(top: Size, right: Size, bottom: Size, left: Size) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn from_3d(top: Size, horizontal: Size, bottom: Size) -> Self {
        Self::from_4d(top, horizontal.clone(), bottom, horizontal.clone())
    }

    pub fn from_2d(vertical: Size, horizontal: Size) -> Self {
        Self::from_4d(
            vertical.clone(),
            horizontal.clone(),
            vertical.clone(),
            horizontal.clone(),
        )
    }

    pub fn from_1d(size: Size) -> Self {
        Self::from_4d(size.clone(), size.clone(), size.clone(), size.clone())
    }

    pub fn get(&self, direction: &str) -> Option<Size> {
        match direction {
            "left" => Some(self.left.clone()),
            "right" => Some(self.right.clone()),
            "top" => Some(self.top.clone()),
            "bottom" => Some(self.bottom.clone()),
            _ => None,
        }
    }
}

impl FromStr for Spacing {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let mut sizes: Vec<Size> = vec![];
        for section in input.split(" ") {
            sizes.push(Size::from_str(section)?);
        }
        match sizes.len() {
            0 => Err(Error::SpacingError(ParseSpacingError::NoValueProvided)),
            1 => Ok(Spacing::from_1d(sizes.get(0).unwrap().clone())),
            2 => Ok(Spacing::from_2d(
                sizes.get(0).unwrap().clone(),
                sizes.get(1).unwrap().clone(),
            )),
            3 => Ok(Spacing::from_3d(
                sizes.get(0).unwrap().clone(),
                sizes.get(1).unwrap().clone(),
                sizes.get(2).unwrap().clone(),
            )),
            4 => Ok(Spacing::from_4d(
                sizes.get(0).unwrap().clone(),
                sizes.get(1).unwrap().clone(),
                sizes.get(2).unwrap().clone(),
                sizes.get(3).unwrap().clone(),
            )),
            _ => Err(Error::SpacingError(
                ParseSpacingError::TooManyValuesProvided,
            )),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn single_value() {
        let res: Spacing = "1px".parse().unwrap();
        assert_eq!(res.top, Size::Pixel(1.0));
        assert_eq!(res.top, res.bottom);
        assert_eq!(res.top, res.right);
        assert_eq!(res.right, res.left);
    }

    #[test]
    fn two_values() {
        let res: Spacing = "2px 4px".parse().unwrap();
        assert_eq!(res.top, Size::Pixel(2.0));
        assert_eq!(res.top, res.bottom);
        assert_eq!(res.left, Size::Pixel(4.0));
        assert_eq!(res.left, res.right);
    }

    #[test]
    fn three_values() {
        let res: Spacing = "2px 3px 4px".parse().unwrap();
        assert_eq!(res.top, Size::Pixel(2.0));
        assert_eq!(res.right, Size::Pixel(3.0));
        assert_eq!(res.left, res.right);
        assert_eq!(res.bottom, Size::Pixel(4.0));
    }

    #[test]
    fn foor_values() {
        let res: Spacing = "2px 3px 4px 5px".parse().unwrap();
        assert_eq!(res.top, Size::Pixel(2.0));
        assert_eq!(res.right, Size::Pixel(3.0));
        assert_eq!(res.bottom, Size::Pixel(4.0));
        assert_eq!(res.left, Size::Pixel(5.0));
    }

    #[test]
    fn more_values() {
        let res = "2px 3px 4px 5px 6px".parse::<Spacing>();
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn invalid_values() {
        let res = "2tx 3px 4px 5px".parse::<Spacing>();
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn accessor() {
        let res = "2px 3px 4px 5px".parse::<Spacing>().unwrap();
        assert_eq!(res.get("top"), Some(Size::Pixel(2.0)));
        assert_eq!(res.get("right"), Some(Size::Pixel(3.0)));
        assert_eq!(res.get("bottom"), Some(Size::Pixel(4.0)));
        assert_eq!(res.get("left"), Some(Size::Pixel(5.0)));
        assert_eq!(res.get("out"), None);
    }
}
