use super::HeadElement;
use crate::mjml::head::prelude::HeadComponent;
use crate::mjml::prelude::*;
use crate::mjml::Error;
use crate::util::fonts::{url_to_import, url_to_link};
use crate::util::{Context, Header, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct MJHead {
    options: Options,
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<HeadElement>,
}

impl MJHead {
    pub fn empty(opts: &Options) -> MJHead {
        debug!("create empty");
        MJHead {
            options: opts.clone(),
            attributes: HashMap::new(),
            context: None,
            children: vec![],
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJHead, Error> {
        let children = HeadElement::parse_all(node.children().collect(), opts)?;
        Ok(MJHead {
            options: opts.clone(),
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }

    pub fn update_header(&self, header: &mut Header) {
        for item in self.children.iter() {
            item.update_header(header);
        }
    }

    pub fn get_title(&self, header: &Header) -> String {
        match header.title() {
            Some(value) => value.clone(),
            None => "".into(),
        }
    }

    pub fn get_preview(&self, _header: &Header) -> String {
        for child in self.children.iter() {
            match child {
                HeadElement::MJPreview(element) => return element.get_content(),
                _ => (),
            };
        }
        "".into()
    }

    fn get_media_queries(&self, header: &Header) -> String {
        if !header.has_media_queries() {
            return "".into();
        }
        let mut res = vec![];
        res.push(format!(
            "@media only screen and (min-width:{}) {{ ",
            header.breakpoint().to_string()
        ));
        let mut classnames: Vec<&String> = header.get_media_queries().keys().collect();
        classnames.sort();
        for classname in classnames.iter() {
            let size = header.get_media_queries().get(classname.clone()).unwrap();
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

    fn get_font_families(&self, header: &Header) -> String {
        let tag = Tag::new("style").set_attribute("type", "text/css");
        let font_urls = header.get_used_font_families();
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

    fn get_styles(&self, header: &Header) -> String {
        let styles = header.get_styles();
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

    fn render(&self, header: &Header) -> Result<String, Error> {
        debug!("render");
        let head = Tag::new("head");
        let mut res: Vec<String> = vec![];
        res.push(head.open());
        res.push(Tag::new("title").render(self.get_title(header)));
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
        res.push(self.get_font_families(header));
        res.push(self.get_media_queries(header));
        res.push(self.get_styles(header));
        res.push(head.close());
        Ok(res.join(""))
    }
}
