use regex::Regex;
use std::cmp::PartialEq;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub enum ParseSizeError {
    Invalid,
}

/// representation of size
///
/// ```rust
/// use mrml::util::size::Size;
/// let size = Size::Percent(12.34);
/// assert_eq!(size.value(), 12.34);
/// ```
#[derive(Clone, Debug)]
pub enum Size {
    Percent(f32),
    Pixel(f32),
    Raw(f32),
}

impl ToString for Size {
    fn to_string(&self) -> String {
        match self {
            Size::Percent(value) => format!("{}%", value),
            Size::Pixel(value) => format!("{}px", value),
            Size::Raw(value) => format!("{}", value),
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value() && self.same_type(other)
    }
}

impl Size {
    pub fn same_type(&self, other: &Self) -> bool {
        (self.is_percent() && other.is_percent())
            || (self.is_pixel() && other.is_pixel())
            || (self.is_raw() && other.is_raw())
    }

    pub fn is_raw(&self) -> bool {
        matches!(self, Size::Raw(_))
    }

    pub fn is_percent(&self) -> bool {
        matches!(self, Size::Percent(_))
    }

    pub fn is_pixel(&self) -> bool {
        matches!(self, Size::Pixel(_))
    }

    fn parse_pixel(input: &str) -> Option<Size> {
        let re = Regex::new(r"^(\d+)px$").unwrap();
        re.captures(input)
            .and_then(|list| list.get(1))
            .and_then(|first| first.as_str().parse::<f32>().ok())
            .map(Size::Pixel)
    }

    fn parse_percent(input: &str) -> Option<Size> {
        let re = Regex::new(r"^(\d+)%$").unwrap();
        re.captures(input)
            .and_then(|list| list.get(1))
            .and_then(|first| first.as_str().parse::<f32>().ok())
            .map(Size::Percent)
    }

    fn parse_raw(input: &str) -> Option<Size> {
        input.parse::<f32>().ok().map(Size::Raw)
    }

    pub fn value(&self) -> f32 {
        match self {
            Size::Percent(value) => *value,
            Size::Pixel(value) => *value,
            Size::Raw(value) => *value,
        }
    }
}

impl FromStr for Size {
    type Err = ParseSizeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match Size::parse_pixel(input)
            .or_else(|| Size::parse_percent(input))
            .or_else(|| Size::parse_raw(input))
        {
            Some(value) => Ok(value),
            None => Err(ParseSizeError::Invalid),
        }
    }
}
