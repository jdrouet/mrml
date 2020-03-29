use super::MJHead;
use crate::elements::head::HeadElement;
use crate::elements::prelude::*;
use crate::elements::Error;
use crate::util::context::Context;
use crate::util::fonts::{url_to_import, url_to_link};
use crate::util::header::Header;
use crate::util::tag::Tag;
use log::debug;

const STYLE_BASE: &str = r#"
<style type="text/css">
#outlook a {
  padding: 0;
}
body {
  margin: 0;
  padding: 0;
  -webkit-text-size-adjust: 100%;
  -ms-text-size-adjust: 100%;
}
table, td {
  border-collapse: collapse;
  mso-table-lspace: 0pt;
  mso-table-rspace: 0pt;
}
img {
  border: 0;
  height: auto;
  line-height: 100%;
  outline: none;
  text-decoration: none;
  -ms-interpolation-mode: bicubic;
}
p {
  display: block;
  margin: 13px 0;
}
</style>
<!--[if mso]>
<xml>
<o:OfficeDocumentSettings>
  <o:AllowPNG/>
  <o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
<![endif]-->
<!--[if lte mso 11]>
<style type="text/css">
.mj-outlook-group-fix { width:100% !important; }
</style>
<![endif]-->
"#;

impl MJHead {
    pub fn get_title(&self) -> String {
        self.header.title.as_ref().cloned().unwrap_or_default()
    }

    pub fn get_preview(&self) -> String {
        for child in self.children.iter() {
            if let HeadElement::MJPreview(element) = child {
                return element.content.clone();
            }
        }
        String::new()
    }

    fn get_media_queries(&self) -> String {
        if !self.header.has_media_queries() {
            return "".into();
        }
        let mut res = vec![];
        res.push(format!(
            "@media only screen and (min-width:{}) {{ ",
            self.header.breakpoint.to_string()
        ));
        let mut classnames: Vec<&String> = self.header.media_queries.keys().collect();
        classnames.sort();
        for classname in classnames.iter() {
            let size = self
                .header
                .media_queries
                .get(&(*classname).clone())
                .unwrap();
            res.push(format!(
                ".{} {{ width:{} !important; max-width: {}; }}",
                classname,
                size.to_string(),
                size.to_string()
            ));
        }
        res.push("}".into());
        Tag::new("style")
            .set_attribute("type", "text/css")
            .render(res.join("\n"))
    }

    fn get_font_families(&self) -> String {
        let tag = Tag::new("style").set_attribute("type", "text/css");
        let font_urls = self.header.get_used_font_families();
        if font_urls.is_empty() {
            return "".into();
        }
        let mut res = vec![];
        res.push("<!--[if !mso]><!-->".into());
        for url in font_urls.iter() {
            res.push(url_to_link(url.as_str()));
        }
        res.push(tag.open());
        for url in font_urls.iter() {
            res.push(url_to_import(url.as_str()));
        }
        res.push(tag.close());
        res.push("<!--<![endif]-->".into());
        res.join("")
    }

    fn get_styles(&self) -> String {
        let styles = self.header.get_styles();
        if styles.is_empty() {
            "".into()
        } else {
            Tag::new("style")
                .set_attribute("type", "text/css")
                .render(styles.join(""))
        }
    }
}

impl Component for MJHead {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        debug!("render");
        let head = Tag::new("head");
        let mut res: Vec<String> = vec![];
        res.push(head.open());
        res.push(Tag::new("title").render(self.get_title()));
        res.push("<!--[if !mso]><!-- -->".into());
        res.push(
            Tag::new("meta")
                .set_attribute("http-equiv", "X-UA-Compatible")
                .set_attribute("content", "IE=edge")
                .open(),
        );
        res.push("<!--<![endif]-->".into());
        res.push(
            Tag::new("meta")
                .set_attribute("http-equiv", "Content-Type")
                .set_attribute("content", "text/html; charset=UTF-8")
                .open(),
        );
        res.push(
            Tag::new("meta")
                .set_attribute("name", "viewport")
                .set_attribute("content", "width=device-width, initial-scale=1")
                .open(),
        );
        res.push(STYLE_BASE.into());
        res.push(self.get_font_families());
        res.push(self.get_media_queries());
        res.push(self.get_styles());
        res.push(head.close());
        Ok(res.join(""))
    }
}
