use super::prelude::{get_node_attributes, Component};
use super::Error;
use crate::util::condition::{END_NEGATION_CONDITIONAL_TAG, START_MSO_NEGATION_CONDITIONAL_TAG};
use crate::util::fonts::{url_to_import, url_to_link};
use crate::util::{Context, Header};
use crate::{close_tag, open_tag, to_attributes, with_tag};
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
    attributes: HashMap<String, String>,
    context: Option<Context>,
    header: Header,
}

impl MJHead {
    pub fn empty() -> MJHead {
        debug!("create empty");
        MJHead {
            attributes: HashMap::new(),
            context: None,
            header: Header::new(),
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJHead, Error> {
        debug!("parse");
        Ok(MJHead {
            attributes: get_node_attributes(&node),
            context: None,
            header: Header::new(),
        })
    }

    pub fn set_header(&mut self, header: Header) {
        self.header.merge(&header);
    }

    fn get_title(&self) -> String {
        match self.header.title() {
            Some(value) => value.clone(),
            None => "".into(),
        }
    }

    fn get_media_queries(&self) -> String {
        let breakpoint = match self
            .context()
            .and_then(|ctx| Some(ctx.options().breakpoint.clone()))
        {
            Some(value) => value,
            None => return "".into(),
        };
        if !self.header.has_media_queries() {
            return "".into();
        }
        let mut res = vec![];
        res.push(format!(
            "@media only screen and (min-width:{}) {{ ",
            breakpoint.to_string()
        ));
        let mut classnames: Vec<&String> = self.header.get_media_queries().keys().collect();
        classnames.sort();
        for classname in classnames.iter() {
            let size = self
                .header
                .get_media_queries()
                .get(classname.clone())
                .unwrap();
            res.push(format!(
                ".{} {{ width:{} !important; max-width: {}; }}",
                classname,
                size.to_string(),
                size.to_string()
            ));
        }
        res.push("}".into());
        with_tag!(
            "style",
            to_attributes!(("type", "text/css")),
            res.join("\n")
        )
    }

    fn get_font_families(&self) -> String {
        let ctx = match self.context() {
            Some(ctx) => ctx,
            None => return "".into(),
        };
        let fonts = self.header.get_font_families();
        let mut font_urls = vec![];
        for font in fonts.iter() {
            if let Some(url) = ctx.options().fonts.get(&font) {
                font_urls.push(url);
            }
        }
        if font_urls.is_empty() {
            return "".into();
        }
        let mut res = vec![];
        res.push(START_MSO_NEGATION_CONDITIONAL_TAG.into());
        for url in font_urls.iter() {
            res.push(url_to_link(url.as_str()));
        }
        res.push(open_tag!("style", to_attributes!(("type", "text/css"))));
        for url in font_urls.iter() {
            res.push(url_to_import(url.as_str()));
        }
        res.push(close_tag!("style"));
        res.push(END_NEGATION_CONDITIONAL_TAG.into());
        res.join("")
    }

    fn get_styles(&self) -> String {
        let styles = self.header.get_styles();
        if styles.is_empty() {
            return "".into();
        }
        let mut res = vec![];
        res.push(open_tag!("style", to_attributes!(("type", "text/css"))));
        for item in styles.iter() {
            res.push(item.to_string());
        }
        res.push(close_tag!("style"));
        res.join("")
    }
}

impl Component for MJHead {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        None
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self) -> Result<String, Error> {
        debug!("render");
        let mut res: Vec<String> = vec![];
        res.push(open_tag!("head"));
        res.push(open_tag!("title"));
        res.push(self.get_title());
        res.push(close_tag!("title"));
        res.push("<!--[if !mso]><!-- -->".into());
        res.push(open_tag!(
            "meta",
            to_attributes!(("http-equiv", "X-UA-Compatible"), ("content", "IE=edge"))
        ));
        res.push("<!--<![endif]-->".into());
        res.push(open_tag!(
            "meta",
            to_attributes!(
                ("http-equiv", "Content-Type"),
                ("content", "text/html; charset=UTF-8")
            )
        ));
        res.push(open_tag!(
            "meta",
            to_attributes!(
                ("name", "viewport"),
                ("content", "width=device-width, initial-scale=1")
            )
        ));
        res.push(STYLE_BASE.into());
        res.push(self.get_font_families());
        res.push(self.get_media_queries());
        res.push(self.get_styles());
        res.push(close_tag!("head"));
        Ok(res.join(""))
    }
}
