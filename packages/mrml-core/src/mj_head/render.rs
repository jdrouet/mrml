use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjHead, MjHeadChild};
use crate::helper::condition::{END_NEGATION_CONDITIONAL_TAG, START_MSO_NEGATION_CONDITIONAL_TAG};
use crate::helper::sort::sort_by_key;
use crate::mj_attributes::MjAttributes;
use crate::mj_include::head::MjIncludeHeadKind;
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

fn select_mj_styles<'a>(buffer: &mut Vec<&'a str>, children: &'a [MjHeadChild]) {
    for child in children.iter() {
        if let Some(mj_style) = child.as_mj_style() {
            buffer.push(mj_style.children());
        } else if let Some(mj_include) = child.as_mj_include() {
            if let MjIncludeHeadKind::Css { inline: false } = mj_include.attributes.kind {
                mj_include
                    .children
                    .iter()
                    .filter_map(|c| c.as_text())
                    .for_each(|c| buffer.push(c.inner_str()));
            }
        }
    }
}

fn extract_attributes_all<'a>(
    result: Map<&'a str, &'a str>,
    attrs: &'a MjAttributes,
) -> Map<&'a str, &'a str> {
    attrs
        .children()
        .iter()
        .filter_map(|item| item.as_mj_attributes_all())
        .fold(result, |mut res, all| {
            res.extend(
                all.attributes()
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str())),
            );
            res
        })
}

fn extract_attributes_class<'a>(
    result: Map<&'a str, Map<&'a str, &'a str>>,
    attrs: &'a MjAttributes,
) -> Map<&'a str, Map<&'a str, &'a str>> {
    attrs
        .children()
        .iter()
        .filter_map(|item| item.as_mj_attributes_class())
        .fold(result, |mut res, class| {
            (*res.entry(class.name()).or_default()).extend(
                class
                    .attributes()
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str())),
            );
            res
        })
}

fn extract_attributes_element<'a>(
    result: Map<&'a str, Map<&'a str, &'a str>>,
    attrs: &'a MjAttributes,
) -> Map<&'a str, Map<&'a str, &'a str>> {
    attrs
        .children()
        .iter()
        .filter_map(|item| item.as_mj_attributes_element())
        .fold(result, |mut res, element| {
            (*res.entry(element.name()).or_default()).extend(
                element
                    .attributes()
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str())),
            );
            res
        })
}

impl MjHead {
    pub fn build_attributes_all(&self) -> Map<&str, &str> {
        // First resolve attributes in <mj-include> tags
        let init = self
            .children
            .iter()
            .filter_map(|item| item.as_mj_include())
            .flat_map(|item| item.children.iter())
            .filter_map(|item| item.as_mj_attributes())
            .fold(Map::<&str, &str>::new(), extract_attributes_all);

        self.children
            .iter()
            .filter_map(|item| item.as_mj_attributes())
            .fold(init, extract_attributes_all)
    }

    pub fn build_attributes_class(&self) -> Map<&str, Map<&str, &str>> {
        // First resolve attributes in <mj-include> tags
        let init = self
            .children
            .iter()
            .filter_map(|item| item.as_mj_include())
            .flat_map(|item| item.children.iter())
            .filter_map(|item| item.as_mj_attributes())
            .fold(
                Map::<&str, Map<&str, &str>>::new(),
                extract_attributes_class,
            );

        self.children
            .iter()
            .filter_map(|item| item.as_mj_attributes())
            .fold(init, extract_attributes_class)
    }

    pub fn build_attributes_element(&self) -> Map<&str, Map<&str, &str>> {
        // First resolve attributes in <mj-include> tags
        let init = self
            .children
            .iter()
            .filter_map(|item| item.as_mj_include())
            .flat_map(|item| item.children.iter())
            .filter_map(|item| item.as_mj_attributes())
            .fold(
                Map::<&str, Map<&str, &str>>::new(),
                extract_attributes_element,
            );

        self.children
            .iter()
            .filter_map(|item| item.as_mj_attributes())
            .fold(init, extract_attributes_element)
    }

    pub fn build_font_families(&self) -> Map<&str, &str> {
        let init = self
            .children
            .iter()
            .filter_map(|item| item.as_mj_include())
            .flat_map(|item| item.children.iter())
            .filter_map(|item| item.as_mj_font())
            .map(|item| (item.name(), item.href()));

        self.children
            .iter()
            .filter_map(|item| item.as_mj_font())
            .map(|item| (item.name(), item.href()))
            .chain(init)
            .collect::<Map<&str, &str>>()
    }
}

pub struct MjHeadRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjHead,
}

impl<'e, 'h> MjHeadRender<'e, 'h> {
    fn render_font_import(&self, href: &str) -> String {
        format!("@import url({href});")
    }

    fn render_font_link(&self, href: &str) -> String {
        format!("<link href=\"{href}\" rel=\"stylesheet\" type=\"text/css\">")
    }

    fn render_font_families(&self, opts: &RenderOptions) -> String {
        let header = self.header.borrow();
        let used_font_families = header.used_font_families();
        if used_font_families.is_empty() {
            return String::default();
        }
        let mut links = String::default();
        header
            .used_font_families()
            .iter()
            .filter_map(|name| opts.fonts.get(name))
            .for_each(|href| links.push_str(&self.render_font_link(href)));
        header
            .used_font_families()
            .iter()
            .filter_map(|name| header.font_families().get(name.as_str()))
            .for_each(|href| links.push_str(&self.render_font_link(href)));
        let mut imports = String::default();
        header
            .used_font_families()
            .iter()
            .filter_map(|name| opts.fonts.get(name))
            .for_each(|href| imports.push_str(&self.render_font_import(href)));
        header
            .used_font_families()
            .iter()
            .filter_map(|name| header.font_families().get(name.as_str()))
            .for_each(|href| imports.push_str(&self.render_font_import(href)));
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
            let mut buffer = Vec::default();
            select_mj_styles(&mut buffer, &self.element.children);
            let buffer = buffer.join("\n");
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
    crate::should_render!(attributes_basic, "mj-attributes");
    crate::should_render!(style_basic, "mj-style");
}
