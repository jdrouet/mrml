use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::MjHead;
use crate::helper::condition::{END_NEGATION_CONDITIONAL_TAG, START_MSO_NEGATION_CONDITIONAL_TAG};
use crate::helper::sort::sort_by_key;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

const STYLE_BASE: &str = r#"
<style type="text/css">
#outlook a { padding: 0; }
body { margin: 0; padding: 0; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
table, td { border-collapse: collapse; mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
img { border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; -ms-interpolation-mode: bicubic; }
p { display: block; margin: 13px 0; }
</style>
<!--[if mso]>
<noscript>
<xml>
<o:OfficeDocumentSettings>
  <o:AllowPNG/>
  <o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
</noscript>
<![endif]-->
<!--[if lte mso 11]>
<style type="text/css">
.mj-outlook-group-fix { width:100% !important; }
</style>
<![endif]-->
"#;

fn combine_attribute_map<'a>(
    mut res: Map<&'a str, Map<&'a str, &'a str>>,
    (name, key, value): (&'a str, &'a str, &'a str),
) -> Map<&'a str, Map<&'a str, &'a str>> {
    let entry = res.entry(name).or_default();
    entry.insert(key, value);
    res
}

impl MjHead {
    pub fn build_attributes_all(&self) -> Map<&str, &str> {
        self.children
            .iter()
            .flat_map(|item| {
                item.as_mj_attributes()
                    .into_iter()
                    .flat_map(|inner| inner.mj_attributes_all_iter())
                    .chain(
                        item.as_mj_include()
                            .filter(|item| item.attributes.kind.is_mjml())
                            .into_iter()
                            .flat_map(|inner| inner.mj_attributes_all_iter()),
                    )
            })
            .collect()
    }

    pub fn build_attributes_class(&self) -> Map<&str, Map<&str, &str>> {
        self.children
            .iter()
            .flat_map(|item| {
                item.as_mj_attributes()
                    .into_iter()
                    .flat_map(|inner| inner.mj_attributes_class_iter())
                    .chain(
                        item.as_mj_include()
                            .filter(|item| item.attributes.kind.is_mjml())
                            .into_iter()
                            .flat_map(|inner| inner.mj_attributes_class_iter()),
                    )
            })
            .fold(Map::new(), combine_attribute_map)
    }

    pub fn build_attributes_element(&self) -> Map<&str, Map<&str, &str>> {
        self.children
            .iter()
            .flat_map(|item| {
                item.as_mj_attributes()
                    .into_iter()
                    .flat_map(|inner| inner.mj_attributes_element_iter())
                    .chain(
                        item.as_mj_include()
                            .filter(|item| item.attributes.kind.is_mjml())
                            .into_iter()
                            .flat_map(|inner| inner.mj_attributes_element_iter()),
                    )
            })
            .fold(Map::new(), combine_attribute_map)
    }

    pub fn build_font_families(&self) -> Map<&str, &str> {
        self.children
            .iter()
            .flat_map(|item| {
                item.as_mj_font().into_iter().chain(
                    item.as_mj_include().into_iter().flat_map(|incl| {
                        incl.children.iter().filter_map(|child| child.as_mj_font())
                    }),
                )
            })
            .map(|font| (font.name(), font.href()))
            .collect()
    }
}

fn render_font_import(target: &mut String, href: &str) {
    target.push_str("@import url(");
    target.push_str(href);
    target.push_str(");");
}

fn render_font_link(target: &mut String, href: &str) {
    target.push_str("<link href=\"");
    target.push_str(href);
    target.push_str("\" rel=\"stylesheet\" type=\"text/css\">");
}

pub struct MjHeadRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjHead,
}

impl<'e, 'h> MjHeadRender<'e, 'h> {
    fn mj_style_iter(&self) -> impl Iterator<Item = &str> {
        self.element.children.iter().flat_map(|item| {
            item.as_mj_include()
                .into_iter()
                .flat_map(|inner| {
                    inner
                        .children
                        .iter()
                        .filter_map(|child| child.as_mj_style())
                        .map(|child| child.children.trim())
                })
                .chain(
                    item.as_mj_include()
                        .into_iter()
                        .filter(|child| child.attributes.kind.is_css(false))
                        .flat_map(|child| {
                            child
                                .children
                                .iter()
                                .filter_map(|item| item.as_text())
                                .map(|text| text.inner_str().trim())
                        }),
                )
                .chain(
                    item.as_mj_style()
                        .into_iter()
                        .map(|item| item.children.trim()),
                )
        })
    }

    fn render_font_families(&self, opts: &RenderOptions) -> String {
        let header = self.header.borrow();
        let used_font_families = header.used_font_families();
        if used_font_families.is_empty() {
            return String::default();
        }

        let mut links = String::default();
        let mut imports = String::default();
        for name in header.used_font_families().iter() {
            if let Some(href) = header.font_families().get(name.as_str()) {
                render_font_link(&mut links, href);
                render_font_import(&mut imports, href);
            } else if let Some(href) = opts.fonts.get(name) {
                render_font_link(&mut links, href);
                render_font_import(&mut imports, href);
            } else {
                // TODO log a warning
            }
        }

        if links.is_empty() && imports.is_empty() {
            String::default()
        } else {
            let mut buf = String::from(START_MSO_NEGATION_CONDITIONAL_TAG);
            buf.push_str(&links);
            if !imports.is_empty() {
                buf.push_str("<style type=\"text/css\">");
                buf.push_str(&imports);
                buf.push_str("</style>");
            }
            buf.push_str(END_NEGATION_CONDITIONAL_TAG);
            buf
        }
    }

    fn render_media_queries(&self) -> String {
        let header = self.header.borrow();
        if header.media_queries().is_empty() {
            return String::default();
        }
        let mut classnames = header.media_queries().iter().collect::<Vec<_>>();
        classnames.sort_by(sort_by_key);
        let breakpoint = header.breakpoint().to_string();
        let mut buf = String::from("<style type=\"text/css\">");
        buf.push_str("@media only screen and (min-width:");
        buf.push_str(breakpoint.as_str());
        buf.push_str(") { ");
        classnames.iter().for_each(|(classname, size)| {
            let size = size.to_string();
            buf.push('.');
            buf.push_str(classname);
            buf.push_str(" { width:");
            buf.push_str(size.as_str());
            buf.push_str(" !important; max-width:");
            buf.push_str(size.as_str());
            buf.push_str("; } ");
        });
        buf.push_str(" }");
        buf.push_str("</style>");
        buf.push_str("<style media=\"screen and (min-width:");
        buf.push_str(breakpoint.as_str());
        buf.push_str(")\">");
        classnames.iter().for_each(|(classname, size)| {
            let size = size.to_string();
            buf.push_str(".moz-text-html .");
            buf.push_str(classname);
            buf.push_str(" { width:");
            buf.push_str(size.as_str());
            buf.push_str(" !important; max-width:");
            buf.push_str(size.as_str());
            buf.push_str("; } ");
        });
        buf.push_str("</style>");
        buf
    }

    fn render_styles(&self) -> String {
        let header = self.header.borrow();
        let header_styles = {
            if header.styles().is_empty() {
                String::default()
            } else {
                let mut buf = "<style type=\"text/css\">".to_string();
                header.styles().iter().for_each(|style| {
                    buf.push_str(style);
                });
                buf.push_str("</style>");
                buf
            }
        };
        let head_styles = {
            let buffer = itertools::join(self.mj_style_iter(), "\n");
            if buffer.is_empty() {
                buffer
            } else {
                format!("<style type=\"text/css\">{buffer}</style>")
            }
        };
        format!("{header_styles}\n{head_styles}").trim().to_string()
    }

    fn render_raw(&self, opts: &RenderOptions) -> Result<String, Error> {
        let mut buffer: Vec<String> = Vec::new();
        let siblings = self.element.children.len();
        for child in self.element.children.iter() {
            if let Some(mj_raw) = child.as_mj_raw() {
                let mut renderer = mj_raw.renderer(Rc::clone(&self.header));
                renderer.set_index(buffer.len());
                renderer.set_siblings(siblings);
                buffer.push(renderer.render(opts)?);
            } else if let Some(mj_include) = child.as_mj_include() {
                for include_child in mj_include.children.iter() {
                    if let Some(mj_raw) = include_child.as_mj_raw() {
                        let mut renderer = mj_raw.renderer(Rc::clone(&self.header));
                        renderer.set_index(buffer.len());
                        renderer.set_siblings(siblings);
                        buffer.push(renderer.render(opts)?);
                    }
                }
            }
        }
        Ok(buffer.join(""))
    }
}

impl<'e, 'h> Render<'h> for MjHeadRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let mut buf = String::from("<head>");
        // we write the title even though there is no content
        buf.push_str("<title>");
        buf.push_str(
            self.element
                .title()
                .map(|item| item.content())
                .unwrap_or_default(),
        );
        buf.push_str("</title>");
        buf.push_str(START_MSO_NEGATION_CONDITIONAL_TAG);
        buf.push_str("<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">");
        buf.push_str(END_NEGATION_CONDITIONAL_TAG);
        buf.push_str("<meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\">");
        buf.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        buf.push_str(STYLE_BASE);
        buf.push_str(&self.render_font_families(opts));
        buf.push_str(&self.render_media_queries());
        buf.push_str(&self.render_styles());
        buf.push_str(&self.render_raw(opts)?);
        buf.push_str("</head>");
        Ok(buf)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjHead {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjHeadRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::{
        mj_attributes::{MjAttributes, MjAttributesChild},
        mj_attributes_all::MjAttributesAll,
        mj_attributes_class::MjAttributesClass,
        mj_attributes_element::MjAttributesElement,
        mj_font::MjFont,
        mj_head::{MjHead, MjHeadChild},
        mj_include::head::{MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild},
        prelude::hash::Map,
    };

    crate::should_render!(attributes_basic, "mj-attributes");
    crate::should_render!(style_basic, "mj-style");

    #[test]
    fn should_keep_order_with_mj_include_attributes_all() {
        let element = MjHead {
            children: vec![
                MjHeadChild::MjAttributes(MjAttributes {
                    children: vec![MjAttributesChild::MjAttributesAll(MjAttributesAll {
                        attributes: Map::from_iter([(
                            String::from("font-size"),
                            String::from("42px"),
                        )]),
                    })],
                }),
                MjHeadChild::MjInclude(MjIncludeHead {
                    attributes: MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    children: vec![MjIncludeHeadChild::MjAttributes(MjAttributes {
                        children: vec![MjAttributesChild::MjAttributesAll(MjAttributesAll {
                            attributes: Map::from_iter([
                                (String::from("font-size"), String::from("21px")),
                                (String::from("text-align"), String::from("center")),
                            ]),
                        })],
                    })],
                }),
                MjHeadChild::MjAttributes(MjAttributes {
                    children: vec![MjAttributesChild::MjAttributesAll(MjAttributesAll {
                        attributes: Map::from_iter([(
                            String::from("text-align"),
                            String::from("right"),
                        )]),
                    })],
                }),
            ],
        };
        assert_eq!(
            element.build_attributes_all().get("font-size"),
            Some("21px").as_ref()
        );
        assert_eq!(
            element.build_attributes_all().get("text-align"),
            Some("right").as_ref()
        );
    }

    #[test]
    fn should_keep_order_with_mj_include_attributes_class() {
        let element = MjHead {
            children: vec![
                MjHeadChild::MjAttributes(MjAttributes {
                    children: vec![MjAttributesChild::MjAttributesClass(MjAttributesClass {
                        name: String::from("foo"),
                        attributes: Map::from_iter([(
                            String::from("font-size"),
                            String::from("42px"),
                        )]),
                    })],
                }),
                MjHeadChild::MjInclude(MjIncludeHead {
                    attributes: MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    children: vec![MjIncludeHeadChild::MjAttributes(MjAttributes {
                        children: vec![
                            MjAttributesChild::MjAttributesClass(MjAttributesClass {
                                name: String::from("foo"),
                                attributes: Map::from_iter([(
                                    String::from("font-size"),
                                    String::from("21px"),
                                )]),
                            }),
                            MjAttributesChild::MjAttributesClass(MjAttributesClass {
                                name: String::from("bar"),
                                attributes: Map::from_iter([(
                                    String::from("text-align"),
                                    String::from("center"),
                                )]),
                            }),
                        ],
                    })],
                }),
                MjHeadChild::MjAttributes(MjAttributes {
                    children: vec![MjAttributesChild::MjAttributesClass(MjAttributesClass {
                        name: String::from("bar"),
                        attributes: Map::from_iter([(
                            String::from("text-align"),
                            String::from("left"),
                        )]),
                    })],
                }),
            ],
        };
        let attributes = element.build_attributes_class();
        assert_eq!(
            attributes.get("foo").unwrap().get("font-size"),
            Some("21px").as_ref()
        );
        assert_eq!(
            attributes.get("bar").unwrap().get("text-align"),
            Some("left").as_ref()
        );
    }

    #[test]
    fn should_keep_order_with_mj_include_attributes_element() {
        let element = MjHead {
            children: vec![
                MjHeadChild::MjAttributes(MjAttributes {
                    children: vec![MjAttributesChild::MjAttributesElement(
                        MjAttributesElement {
                            name: String::from("mj-text"),
                            attributes: Map::from_iter([(
                                String::from("font-size"),
                                String::from("42px"),
                            )]),
                        },
                    )],
                }),
                MjHeadChild::MjInclude(MjIncludeHead {
                    attributes: MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    children: vec![MjIncludeHeadChild::MjAttributes(MjAttributes {
                        children: vec![MjAttributesChild::MjAttributesElement(
                            MjAttributesElement {
                                name: String::from("mj-text"),
                                attributes: Map::from_iter([
                                    (String::from("font-size"), String::from("21px")),
                                    (String::from("text-align"), String::from("center")),
                                ]),
                            },
                        )],
                    })],
                }),
                MjHeadChild::MjAttributes(MjAttributes {
                    children: vec![MjAttributesChild::MjAttributesElement(
                        MjAttributesElement {
                            name: String::from("mj-text"),
                            attributes: Map::from_iter([(
                                String::from("text-align"),
                                String::from("left"),
                            )]),
                        },
                    )],
                }),
            ],
        };
        let attributes = element.build_attributes_element();
        assert_eq!(
            attributes.get("mj-text").unwrap().get("font-size"),
            Some("21px").as_ref()
        );
        assert_eq!(
            attributes.get("mj-text").unwrap().get("text-align"),
            Some("left").as_ref()
        );
    }

    #[test]
    fn should_keep_order_with_mj_font() {
        let element = MjHead {
            children: vec![
                MjHeadChild::MjFont(MjFont::new("foo", "http://foo/root")),
                MjHeadChild::MjInclude(MjIncludeHead {
                    attributes: MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    children: vec![
                        MjIncludeHeadChild::MjFont(MjFont::new("foo", "http://foo/include")),
                        MjIncludeHeadChild::MjFont(MjFont::new("bar", "http://bar/include")),
                    ],
                }),
                MjHeadChild::MjFont(MjFont::new("bar", "http://bar/root")),
            ],
        };
        let fonts = element.build_font_families();
        assert_eq!(fonts.get("foo"), Some("http://foo/include").as_ref());
        assert_eq!(fonts.get("bar"), Some("http://bar/root").as_ref());
    }
}
