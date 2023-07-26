use std::collections::{HashMap, HashSet};

use lightningcss::error::{Error as CssError, ParserError};
use lightningcss::properties::font::{FontFamily, FontWeight};
use lightningcss::properties::Property;
use lightningcss::rules::font_face::{FontFaceProperty, FontFaceRule, FontStyle};
use lightningcss::rules::style::StyleRule;
use lightningcss::rules::unknown::UnknownAtRule;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::traits::ToCss;
use lightningcss::values::angle::Angle;

#[derive(Debug)]
pub enum Error<'a> {
    Parser(CssError<ParserError<'a>>),
    MissingStyleProperties {
        path: String,
        rules: Vec<String>,
    },
    UnexpectedProperties {
        path: String,
        rules: Vec<String>,
    },
    MismatchFontFace {
        path: String,
        expected: String,
        generated: String,
    },
    MismatchRules {
        path: String,
        expected: String,
        generated: String,
    },
    MismatchImports {
        path: String,
        expected: String,
        generated: String,
    },
    MissingRules {
        path: String,
        rules: Vec<String>,
    },
    UnexpectedRules {
        path: String,
        rules: Vec<String>,
    },
}

fn font_family_as_key(item: &FontFamily<'_>) -> String {
    match item {
        FontFamily::FamilyName(inner) => inner.to_string(),
        FontFamily::Generic(inner) => inner.as_str().to_string(),
    }
}

fn font_weight_as_key(item: &FontWeight) -> String {
    match item {
        FontWeight::Bolder => "bolder".to_string(),
        FontWeight::Lighter => "lighter".to_string(),
        FontWeight::Absolute(inner) => match inner {
            lightningcss::properties::font::AbsoluteFontWeight::Normal => "normal".into(),
            lightningcss::properties::font::AbsoluteFontWeight::Bold => "bold".into(),
            lightningcss::properties::font::AbsoluteFontWeight::Weight(w) => w.to_string(),
        },
    }
}

fn oblique_angle_as_key(item: &Angle) -> String {
    item.to_css_string(PrinterOptions::default()).unwrap()
}

fn font_face_as_key(item: &FontFaceRule<'_>) -> String {
    let mut res = String::default();
    if let Some(font_family) = item.properties.iter().find_map(|p| match p {
        FontFaceProperty::FontFamily(inner) => Some(font_family_as_key(inner)),
        _ => None,
    }) {
        res.push_str("font-family:");
        res.push_str(&font_family);
        res.push(';');
    }
    if let Some(font_weight) = item.properties.iter().find_map(|p| match p {
        FontFaceProperty::FontWeight(inner) => Some(format!(
            "{} {}",
            font_weight_as_key(&inner.0),
            font_weight_as_key(&inner.1)
        )),
        _ => None,
    }) {
        res.push_str("font-weight:");
        res.push_str(&font_weight);
        res.push(';');
    }
    if let Some(font_style) = item.properties.iter().find_map(|p| match p {
        FontFaceProperty::FontStyle(style) => match style {
            FontStyle::Normal => Some("normal".to_string()),
            FontStyle::Italic => Some("italic".to_string()),
            FontStyle::Oblique(inner) => Some(format!(
                "{} {}",
                oblique_angle_as_key(&inner.0),
                oblique_angle_as_key(&inner.1)
            )),
        },
        _ => None,
    }) {
        res.push_str("font-style:");
        res.push_str(&font_style);
        res.push(';');
    }
    res
}

fn css_rule_as_key<R: std::fmt::Debug + std::cmp::PartialEq>(rule: &CssRule<'_, R>) -> String {
    match rule {
        CssRule::Media(media_inner) => format!(
            "media({})",
            media_inner.query.to_css_string(Default::default()).unwrap()
        ),
        CssRule::Style(inner) => format!(
            "style({})",
            inner
                .selectors
                .0
                .iter()
                .map(|sel| sel.to_css_string(Default::default()).unwrap())
                .collect::<Vec<_>>()
                .join(", "),
        ),
        CssRule::Import(inner) => format!("import({})", inner.url),
        CssRule::Unknown(inner) => format!("unknown({})", inner.name),
        CssRule::FontFace(inner) => format!("font-face({})", font_face_as_key(inner)),
        others => todo!("css_rule_as_key {others:?}"),
    }
}

fn compare_style_properties<'a>(
    path: &str,
    exp: &[Property<'a>],
    gen: &[Property<'a>],
    important: bool,
) -> Result<(), Error<'a>> {
    let exp_props = exp
        .iter()
        .map(|p| {
            p.to_css_string(important, PrinterOptions::default())
                .unwrap()
        })
        .collect::<HashSet<_>>();
    let gen_props = gen
        .iter()
        .map(|p| {
            p.to_css_string(important, PrinterOptions::default())
                .unwrap()
        })
        .collect::<HashSet<_>>();

    let diff = exp_props
        .difference(&gen_props)
        .cloned()
        .collect::<Vec<_>>();

    if !diff.is_empty() {
        return Err(Error::MissingStyleProperties {
            path: path.to_string(),
            rules: diff,
        });
    }

    let diff = gen_props
        .difference(&exp_props)
        .cloned()
        .collect::<Vec<_>>();

    if !diff.is_empty() {
        return Err(Error::UnexpectedProperties {
            path: path.to_string(),
            rules: diff,
        });
    }

    Ok(())
}

fn compare_style<'a, R: std::fmt::Debug + std::cmp::PartialEq>(
    path: &str,
    exp: StyleRule<'a, R>,
    gen: StyleRule<'a, R>,
) -> Result<(), Error<'a>> {
    compare_style_properties(
        path,
        &exp.declarations.declarations,
        &gen.declarations.declarations,
        false,
    )?;
    compare_style_properties(
        path,
        &exp.declarations.important_declarations,
        &gen.declarations.important_declarations,
        true,
    )?;
    Ok(())
}

fn compare_font_face<'a>(
    path: &str,
    exp: FontFaceRule<'a>,
    gen: FontFaceRule<'a>,
) -> Result<(), Error<'a>> {
    let mut exp_props = exp
        .properties
        .iter()
        .map(|prop| prop.to_css_string(PrinterOptions::default()).unwrap())
        .collect::<Vec<_>>();
    exp_props.sort();
    let exp_props = exp_props.join("\n");
    let mut gen_props = gen
        .properties
        .iter()
        .map(|prop| prop.to_css_string(PrinterOptions::default()).unwrap())
        .collect::<Vec<_>>();
    gen_props.sort();
    let gen_props = gen_props.join("\n");
    if exp_props != gen_props {
        Err(Error::MismatchFontFace {
            path: path.to_string(),
            expected: exp_props,
            generated: gen_props,
        })
    } else {
        Ok(())
    }
}

fn compare_unknown<'a>(
    path: &str,
    exp: UnknownAtRule<'a>,
    gen: UnknownAtRule<'a>,
) -> Result<(), Error<'a>> {
    let exp_str = exp.to_css_string(PrinterOptions::default()).unwrap();
    let gen_str = gen.to_css_string(PrinterOptions::default()).unwrap();
    if exp_str != gen_str {
        Err(Error::MismatchRules {
            path: path.to_string(),
            expected: exp_str,
            generated: gen_str,
        })
    } else {
        Ok(())
    }
}

fn compare_rule<'a, R: std::fmt::Debug + std::cmp::PartialEq>(
    path: &str,
    exp: CssRule<'a, R>,
    gen: CssRule<'a, R>,
) -> Result<(), Error<'a>> {
    match (exp, gen) {
        (CssRule::Media(exp), CssRule::Media(gen)) => {
            compare_rules(path, exp.rules.0, gen.rules.0)?;
        }
        (CssRule::Style(exp), CssRule::Style(gen)) => {
            compare_style(path, exp, gen)?;
        }
        (CssRule::Import(exp), CssRule::Import(gen)) => {
            if exp.url != gen.url {
                return Err(Error::MismatchImports {
                    path: path.to_string(),
                    expected: exp.url.to_string(),
                    generated: gen.url.to_string(),
                });
            }
        }
        (CssRule::FontFace(exp), CssRule::FontFace(gen)) => {
            compare_font_face(path, exp, gen)?;
        }
        (CssRule::Unknown(exp), CssRule::Unknown(gen)) => {
            compare_unknown(path, exp, gen)?;
        }
        (exp, gen) => {
            return Err(Error::MismatchRules {
                path: path.to_string(),
                expected: format!("{exp:#?}"),
                generated: format!("{gen:#?}"),
            })
        }
    }
    Ok(())
}

fn compare_rules<'a, R: std::fmt::Debug + std::cmp::PartialEq>(
    path: &str,
    exps: Vec<CssRule<'a, R>>,
    gens: Vec<CssRule<'a, R>>,
) -> Result<(), Error<'a>> {
    let exp_map = exps
        .into_iter()
        .map(|item| (css_rule_as_key(&item), item))
        .collect::<HashMap<_, _>>();
    let gen_map = gens
        .into_iter()
        .map(|item| (css_rule_as_key(&item), item))
        .collect::<HashMap<_, _>>();

    let exp_keys = exp_map.keys().map(|s| s.as_str()).collect::<HashSet<_>>();
    let gen_keys = gen_map.keys().map(|s| s.as_str()).collect::<HashSet<_>>();

    let diff = exp_keys
        .difference(&gen_keys)
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    if !diff.is_empty() {
        return Err(Error::MissingRules {
            path: path.to_string(),
            rules: diff,
        });
    }

    let diff = gen_keys
        .difference(&exp_keys)
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    if !diff.is_empty() {
        return Err(Error::UnexpectedRules {
            path: path.to_string(),
            rules: diff,
        });
    }

    let mut gen_map = gen_map;

    for (key, exp, gen) in exp_map
        .into_iter()
        .filter_map(|(key, exp)| gen_map.remove(&key).map(|gen| (key, exp, gen)))
    {
        let path = format!("{path} > {key}");
        compare_rule(&path, exp, gen)?;
    }

    Ok(())
}

pub fn compare<'a>(expected: &'a str, generated: &'a str) -> Result<(), Error<'a>> {
    let expected_stylesheet =
        StyleSheet::parse(expected, ParserOptions::default()).map_err(Error::Parser)?;
    let generated_stylesheet =
        StyleSheet::parse(generated, ParserOptions::default()).map_err(Error::Parser)?;

    compare_rules(
        "$",
        expected_stylesheet.rules.0,
        generated_stylesheet.rules.0,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn with_media() {
        let expected = r#"@media only screen and (min-width:480px) {
        .mj-column-per-50 {
            width: 50% !important;
            max-width: 50%;
        }
    
        .mj-column-per-33-333332 {
            width: 33.333332% !important;
            max-width: 33.333332%;
        }
    }"#;
        let generated = "@media only screen and (min-width:480px) { .mj-column-per-33-333332 { width:33.333332% !important; max-width:33.333332%; } .mj-column-per-50 { width:50% !important; max-width:50%; }}";

        super::compare(expected, generated).unwrap();
    }

    #[test]
    fn with_media_yahoo() {
        let expected = r#"@media screen, yahoo {
    .mj-carousel-00000000-icons-cell,
    .mj-carousel-previous-icons,
    .mj-carousel-next-icons {
        display: none !important;
    }
    .mj-carousel-00000000-radio-1:checked+*+*+.mj-carousel-content .mj-carousel-00000000-thumbnail-1 {
        border-color: transparent;
    }
}"#;
        let generated = r#"@media screen, yahoo {
        .mj-carousel-00000000-icons-cell,
        .mj-carousel-previous-icons,
        .mj-carousel-next-icons {
            display: none !important;
        }
        .mj-carousel-00000000-radio-1:checked+*+*+.mj-carousel-content .mj-carousel-00000000-thumbnail-1 {
            border-color: transparent;
        }
    }"#;

        super::compare(expected, generated).unwrap();
    }
}
