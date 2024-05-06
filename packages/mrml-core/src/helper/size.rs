use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    Pixel(Pixel),
    Percent(Percent),
    Raw(f32),
}

impl Size {
    pub fn percent(value: f32) -> Self {
        Self::Percent(Percent::new(value))
    }
    pub fn pixel(value: f32) -> Self {
        Self::Pixel(Pixel::new(value))
    }

    pub fn as_percent(&self) -> Option<&Percent> {
        match self {
            Self::Percent(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_percent(&self) -> bool {
        matches!(self, Self::Percent(_))
    }

    pub fn as_pixel(&self) -> Option<&Pixel> {
        match self {
            Self::Pixel(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_pixel(&self) -> bool {
        matches!(self, Self::Pixel(_))
    }

    pub fn value(&self) -> f32 {
        match self {
            Self::Pixel(p) => p.value(),
            Self::Percent(p) => p.value(),
            Self::Raw(v) => *v,
        }
    }

    pub fn from_border(input: &str) -> Option<Self> {
        input
            .split_whitespace()
            .next()
            .and_then(|value| Size::try_from(value).ok())
    }
}

impl TryFrom<&str> for Size {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with("px") {
            Ok(Self::Pixel(Pixel::try_from(value)?))
        } else if value.ends_with('%') {
            Ok(Self::Percent(Percent::try_from(value)?))
        } else {
            Ok(Self::Raw(
                value.parse::<f32>().map_err(|err| err.to_string())?,
            ))
        }
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pixel(inner) => inner.fmt(f),
            Self::Percent(inner) => inner.fmt(f),
            Self::Raw(inner) => write!(f, "{inner}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Percent(f32);

impl Percent {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<&str> for Percent {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(value) = value.strip_suffix('%') {
            value
                .parse::<f32>()
                .map(Percent::new)
                .map_err(|err| err.to_string())
        } else {
            Err(String::from("percent value should end with %"))
        }
    }
}

impl Default for Percent {
    fn default() -> Self {
        Self(0.0)
    }
}

impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pixel(f32);

impl Pixel {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn from_border(input: &str) -> Option<Self> {
        input
            .split_whitespace()
            .next()
            .and_then(|value| Self::try_from(value).ok())
    }

    pub fn lower(&self) -> Self {
        if self.0 <= 1.0 {
            Self(0.0)
        } else {
            Self(self.0 - 1.0)
        }
    }
}

impl TryFrom<&str> for Pixel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(value) = value.strip_suffix("px") {
            value
                .parse::<f32>()
                .map(Pixel::new)
                .map_err(|err| err.to_string())
        } else {
            Err(String::from("pixel value should end with px"))
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self(0.0)
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}px", self.0)
    }
}
