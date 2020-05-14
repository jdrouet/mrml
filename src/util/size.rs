use regex::Regex;
use std::str::FromStr;
use std::string::ToString;

pub enum ParseSizeError {
    Invalid,
}

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

impl Size {
    fn parse_pixel(input: &str) -> Option<Size> {
        let re = Regex::new(r"^(\d+)px$").unwrap();
        re.captures(input)
            .and_then(|list| list.get(1))
            .and_then(|first| first.as_str().parse::<f32>().ok())
            .and_then(|value| Some(Size::Pixel(value)))
    }

    fn parse_percent(input: &str) -> Option<Size> {
        let re = Regex::new(r"^(\d+)%$").unwrap();
        re.captures(input)
            .and_then(|list| list.get(1))
            .and_then(|first| first.as_str().parse::<f32>().ok())
            .and_then(|value| Some(Size::Percent(value)))
    }

    fn parse_raw(input: &str) -> Option<Size> {
        input
            .parse::<f32>()
            .ok()
            .and_then(|value| Some(Size::Raw(value)))
    }

    pub fn value(&self) -> f32 {
        match self {
            Size::Percent(value) => value.clone(),
            Size::Pixel(value) => value.clone(),
            Size::Raw(value) => value.clone(),
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
