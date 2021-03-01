use super::MJHead;
use crate::elements::error::Error;
use crate::elements::prelude::Component;
use crate::util::context::Context;
use crate::util::fonts::{url_to_import, url_to_link};
use crate::util::header::Header;
use crate::util::sort_by_key;
use crate::util::tag::Tag;

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
    fn get_title_from_children(&self) -> Option<String> {
        self.children
            .iter()
            .filter_map(|item| item.as_mj_title())
            .map(|item| item.get_content())
            .next()
    }

    pub fn get_title(&self) -> String {
        self.header
            .title
            .as_ref()
            .cloned()
            .or_else(|| self.get_title_from_children())
            .unwrap_or_default()
    }

    pub fn get_preview(&self) -> String {
        self.children
            .iter()
            .filter_map(|item| item.as_mj_preview())
            .map(|item| item.content.clone())
            .next()
            .unwrap_or_default()
    }

    fn get_media_queries(&self) -> String {
        if !self.header.has_media_queries() {
            return String::default();
        }
        let mut res = vec![format!(
            "@media only screen and (min-width:{}) {{ ",
            self.header.breakpoint.to_string()
        )];
        let mut classnames: Vec<(&String, String)> = self
            .header
            .media_queries
            .iter()
            .map(|(key, value)| (key, value.to_string()))
            .collect();
        classnames.sort_by(sort_by_key);
        classnames.iter().for_each(|(classname, size)| {
            res.push(format!(
                ".{} {{ width:{} !important; max-width: {}; }}",
                classname, size, size
            ))
        });
        res.push("}".into());
        Tag::new("style")
            .set_attribute("type", "text/css")
            .render(res.join("\n"))
    }

    fn get_font_families(&self) -> String {
        let font_urls = self.header.get_used_font_families();
        if font_urls.is_empty() {
            return String::default();
        }
        String::from("<!--[if !mso]><!-->")
            + &font_urls
                .iter()
                .map(|url| url_to_link(url.as_str()))
                .collect::<String>()
            + "<style type=\"text/css\">"
            + &font_urls
                .iter()
                .map(|url| url_to_import(url.as_str()))
                .collect::<String>()
            + "</style>"
            + "<!--<![endif]-->"
    }

    fn get_styles(&self) -> String {
        let styles = self.header.get_styles();
        if styles.is_empty() {
            String::default()
        } else {
            format!("<style type=\"text/css\">{}</style>", styles.join(""))
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
        Ok(String::from("<head>")
            + "<title>"
            + &self.get_title()
            + "</title>"
            + "<!--[if !mso]><!-- -->"
            + "<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">"
            + "<!--<![endif]-->"
            + "<meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\">"
            + "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"
            + STYLE_BASE
            + &self.get_font_families()
            + &self.get_media_queries()
            + &self.get_styles()
            + "</head>")
    }
}
