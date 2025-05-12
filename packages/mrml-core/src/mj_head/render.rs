use super::MjHead;
use crate::helper::sort::sort_by_key;
use crate::prelude::hash::Map;
use crate::prelude::render::*;

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
                            .filter(|item| item.0.attributes.kind.is_mjml())
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
                            .filter(|item| item.0.attributes.kind.is_mjml())
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
                            .filter(|item| item.0.attributes.kind.is_mjml())
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
                item.as_mj_font()
                    .into_iter()
                    .chain(item.as_mj_include().into_iter().flat_map(|incl| {
                        incl.0
                            .children
                            .iter()
                            .filter_map(|child| child.as_mj_font())
                    }))
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

impl Renderer<'_, MjHead, ()> {
    fn mj_style_iter(&self) -> impl Iterator<Item = (&str, bool)> {
        self.element.children.iter().flat_map(|item| {
            // Include styles from mj_include elements
            let included_styles = item.as_mj_include().into_iter().flat_map(|inner| {
                let includes_css = inner
                    .0
                    .children
                    .iter()
                    .filter_map(|child| child.as_mj_style())
                    .map(|child| (child.children.trim(), child.is_inline()));

                let direct_css = inner
                    .0
                    .attributes
                    .kind
                    .is_css(false)
                    .then(|| {
                        inner
                            .0
                            .children
                            .iter()
                            .filter_map(|item| item.as_text())
                            .map(|text| (text.inner_str().trim(), false))
                    })
                    .into_iter()
                    .flatten();

                includes_css.chain(direct_css)
            });

            // Direct mj_style elements
            let direct_styles = item
                .as_mj_style()
                .into_iter()
                .map(|style| (style.children.trim(), style.is_inline()));

            included_styles.chain(direct_styles)
        })
    }

    fn render_font_families(&self, cursor: &mut RenderCursor) {
        let used_font_families = cursor.header.used_font_families();
        if used_font_families.is_empty() {
            return;
        }

        let mut links = String::default();
        let mut imports = String::default();
        for name in cursor.header.used_font_families().iter() {
            if let Some(href) = self.context.header.font_families().get(name.as_str()) {
                render_font_link(&mut links, href);
                render_font_import(&mut imports, href);
            } else if let Some(href) = self.context.options.fonts.get(name) {
                render_font_link(&mut links, href);
                render_font_import(&mut imports, href);
            } else {
                // TODO log a warning
            }
        }

        if links.is_empty() && imports.is_empty() {
        } else {
            cursor.buffer.start_mso_negation_conditional_tag();
            cursor.buffer.push_str(&links);
            if !imports.is_empty() {
                cursor.buffer.push_str("<style type=\"text/css\">");
                cursor.buffer.push_str(&imports);
                cursor.buffer.push_str("</style>");
            }
            cursor.buffer.end_negation_conditional_tag();
        }
    }

    fn render_media_queries(&self, cursor: &mut RenderCursor) {
        if cursor.header.media_queries().is_empty() {
            return;
        }
        let mut classnames = cursor.header.media_queries().iter().collect::<Vec<_>>();
        classnames.sort_by(sort_by_key);
        let breakpoint = self.context.header.breakpoint().to_string();
        cursor.buffer.push_str("<style type=\"text/css\">");
        cursor.buffer.push_str("@media only screen and (min-width:");
        cursor.buffer.push_str(breakpoint.as_str());
        cursor.buffer.push_str(") { ");
        for (classname, size) in classnames.iter() {
            let size = size.to_string();
            cursor.buffer.push('.');
            cursor.buffer.push_str(classname);
            cursor.buffer.push_str(" { width:");
            cursor.buffer.push_str(size.as_str());
            cursor.buffer.push_str(" !important; max-width:");
            cursor.buffer.push_str(size.as_str());
            cursor.buffer.push_str("; } ");
        }
        cursor.buffer.push_str(" }");
        cursor.buffer.push_str("</style>");
        cursor
            .buffer
            .push_str("<style media=\"screen and (min-width:");
        cursor.buffer.push_str(breakpoint.as_str());
        cursor.buffer.push_str(")\">");
        for (classname, size) in classnames.iter() {
            let size = size.to_string();
            cursor.buffer.push_str(".moz-text-html .");
            cursor.buffer.push_str(classname);
            cursor.buffer.push_str(" { width:");
            cursor.buffer.push_str(size.as_str());
            cursor.buffer.push_str(" !important; max-width:");
            cursor.buffer.push_str(size.as_str());
            cursor.buffer.push_str("; } ");
        }
        cursor.buffer.push_str("</style>");
    }

    fn render_styles(&self, cursor: &mut RenderCursor) {
        // Add regular styles from header to the buffer
        if !cursor.header.styles().is_empty() {
            cursor.buffer.push_str("<style type=\"text/css\">");
            for style in cursor.header.styles().iter() {
                cursor.buffer.push_str(style);
            }
            cursor.buffer.push_str("</style>");
        }

        // Process mj-style items
        for (content, is_inline) in self.mj_style_iter() {
            if is_inline {
                // Add inline styles to inline_styles collection
                cursor.header.add_inline_style(content.to_string());
            } else {
                // Output non-inlined styles to the buffer
                cursor.buffer.push_str("<style type=\"text/css\">");
                cursor.buffer.push_str(content);
                cursor.buffer.push_str("</style>");
            }
        }
    }

    fn render_raw(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let mut index: usize = 0;
        let siblings = self.element.children.len();
        for child in self.element.children.iter() {
            if let Some(mj_raw) = child.as_mj_raw() {
                let mut renderer = mj_raw.renderer(self.context());
                renderer.set_index(index);
                renderer.set_siblings(siblings);
                renderer.render(cursor)?;
                index += 1;
            } else if let Some(mj_include) = child.as_mj_include() {
                for include_child in mj_include.0.children.iter() {
                    if let Some(mj_raw) = include_child.as_mj_raw() {
                        let mut renderer = mj_raw.renderer(self.context());
                        renderer.set_index(index);
                        renderer.set_siblings(siblings);
                        renderer.render(cursor)?;
                        index += 1;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'root> Render<'root> for Renderer<'root, MjHead, ()> {
    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        cursor.buffer.push_str("<head>");
        // we write the title even though there is no content
        cursor.buffer.push_str("<title>");
        if let Some(title) = self.element.title().map(|item| item.content()) {
            cursor.buffer.push_str(title);
        }
        cursor.buffer.push_str("</title>");
        cursor.buffer.start_mso_negation_conditional_tag();
        cursor
            .buffer
            .push_str("<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">");
        cursor.buffer.end_negation_conditional_tag();
        cursor
            .buffer
            .push_str("<meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\">");
        cursor
            .buffer
            .push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        cursor.buffer.push_str(STYLE_BASE);
        self.render_font_families(cursor);
        self.render_media_queries(cursor);
        self.render_styles(cursor);
        self.render_raw(cursor)?;
        cursor.buffer.push_str("</head>");
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjHead {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::mj_attributes::{MjAttributes, MjAttributesChild};
    use crate::mj_attributes_all::MjAttributesAll;
    use crate::mj_attributes_class::{MjAttributesClass, MjAttributesClassAttributes};
    use crate::mj_attributes_element::MjAttributesElement;
    use crate::mj_font::MjFont;
    use crate::mj_head::{MjHead, MjHeadChild};
    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild};
    use crate::prelude::hash::Map;

    crate::should_render!(attributes_basic, "mj-attributes");
    crate::should_render!(style_basic, "mj-style");

    #[test]
    fn should_keep_order_with_mj_include_attributes_all() {
        let element = MjHead::new(
            (),
            vec![
                MjHeadChild::MjAttributes(MjAttributes::new(
                    (),
                    vec![MjAttributesChild::MjAttributesAll(MjAttributesAll::new(
                        Map::from_iter([(String::from("font-size"), Some(String::from("42px")))]),
                        (),
                    ))],
                )),
                MjHeadChild::MjInclude(MjIncludeHead::new(
                    MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    vec![MjIncludeHeadChild::MjAttributes(MjAttributes::new(
                        (),
                        vec![MjAttributesChild::MjAttributesAll(MjAttributesAll::new(
                            Map::from_iter([
                                (String::from("font-size"), Some(String::from("21px"))),
                                (String::from("text-align"), Some(String::from("center"))),
                            ]),
                            (),
                        ))],
                    ))],
                )),
                MjHeadChild::MjAttributes(MjAttributes::new(
                    (),
                    vec![MjAttributesChild::MjAttributesAll(MjAttributesAll::new(
                        Map::from_iter([(String::from("text-align"), Some(String::from("right")))]),
                        (),
                    ))],
                )),
            ],
        );
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
        let element = MjHead::new(
            (),
            vec![
                MjHeadChild::MjAttributes(MjAttributes::new(
                    (),
                    vec![MjAttributesChild::MjAttributesClass(
                        MjAttributesClass::new(
                            MjAttributesClassAttributes {
                                name: String::from("foo"),
                                others: Map::from_iter([(
                                    String::from("font-size"),
                                    Some(String::from("42px")),
                                )]),
                            },
                            (),
                        ),
                    )],
                )),
                MjHeadChild::MjInclude(MjIncludeHead::new(
                    MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    vec![MjIncludeHeadChild::MjAttributes(MjAttributes::new(
                        (),
                        vec![
                            MjAttributesChild::MjAttributesClass(MjAttributesClass::new(
                                MjAttributesClassAttributes {
                                    name: String::from("foo"),
                                    others: Map::from_iter([(
                                        String::from("font-size"),
                                        Some(String::from("21px")),
                                    )]),
                                },
                                (),
                            )),
                            MjAttributesChild::MjAttributesClass(MjAttributesClass::new(
                                MjAttributesClassAttributes {
                                    name: String::from("bar"),
                                    others: Map::from_iter([(
                                        String::from("text-align"),
                                        Some(String::from("center")),
                                    )]),
                                },
                                (),
                            )),
                        ],
                    ))],
                )),
                MjHeadChild::MjAttributes(MjAttributes::new(
                    (),
                    vec![MjAttributesChild::MjAttributesClass(
                        MjAttributesClass::new(
                            MjAttributesClassAttributes {
                                name: String::from("bar"),
                                others: Map::from_iter([(
                                    String::from("text-align"),
                                    Some(String::from("left")),
                                )]),
                            },
                            (),
                        ),
                    )],
                )),
            ],
        );
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
        let element = MjHead::new(
            (),
            vec![
                MjHeadChild::MjAttributes(MjAttributes::new(
                    (),
                    vec![MjAttributesChild::MjAttributesElement(
                        MjAttributesElement {
                            name: String::from("mj-text"),
                            attributes: Map::from_iter([(
                                String::from("font-size"),
                                Some(String::from("42px")),
                            )]),
                        },
                    )],
                )),
                MjHeadChild::MjInclude(MjIncludeHead::new(
                    MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    vec![MjIncludeHeadChild::MjAttributes(MjAttributes::new(
                        (),
                        vec![MjAttributesChild::MjAttributesElement(
                            MjAttributesElement {
                                name: String::from("mj-text"),
                                attributes: Map::from_iter([
                                    (String::from("font-size"), Some(String::from("21px"))),
                                    (String::from("text-align"), Some(String::from("center"))),
                                ]),
                            },
                        )],
                    ))],
                )),
                MjHeadChild::MjAttributes(MjAttributes::new(
                    (),
                    vec![MjAttributesChild::MjAttributesElement(
                        MjAttributesElement {
                            name: String::from("mj-text"),
                            attributes: Map::from_iter([(
                                String::from("text-align"),
                                Some(String::from("left")),
                            )]),
                        },
                    )],
                )),
            ],
        );
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
        let element = MjHead::new(
            (),
            vec![
                MjHeadChild::MjFont(MjFont::build("foo", "http://foo/root")),
                MjHeadChild::MjInclude(MjIncludeHead::new(
                    MjIncludeHeadAttributes {
                        path: String::from("foo"),
                        kind: crate::mj_include::head::MjIncludeHeadKind::Mjml,
                    },
                    vec![
                        MjIncludeHeadChild::MjFont(MjFont::build("foo", "http://foo/include")),
                        MjIncludeHeadChild::MjFont(MjFont::build("bar", "http://bar/include")),
                    ],
                )),
                MjHeadChild::MjFont(MjFont::build("bar", "http://bar/root")),
            ],
        );
        let fonts = element.build_font_families();
        assert_eq!(fonts.get("foo"), Some("http://foo/include").as_ref());
        assert_eq!(fonts.get("bar"), Some("http://bar/root").as_ref());
    }
}
