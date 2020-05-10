use super::{Component, Error};
use crate::{close_tag, open_tag};
use crate::util::Properties;
use log::debug;
use roxmltree::Node;

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
pub struct MJHead<'a, 'b> {
    context: Option<Properties>,
    node: Option<Node<'a, 'b>>,
    title: String,
}

impl MJHead<'_, '_> {
    pub fn empty<'a, 'b>() -> MJHead<'a, 'b> {
        debug!("create empty");
        MJHead {
            context: None,
            node: None,
            title: "".into(),
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJHead<'a, 'b>, Error> {
        debug!("parse");
        Ok(MJHead {
            context: None,
            node: Some(node),
            title: "".into(),
        })
    }
}

impl Component for MJHead<'_, '_> {
    fn default_attribute(key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        None
    }

    fn node(&self) -> Option<Node> {
        self.node
    }

    fn set_context(&mut self, ctx: Properties) {
        self.context = Some(ctx);
    }

    fn render(&self) -> Result<String, Error> {
        debug!("render");
        let mut res: Vec<String> = vec![];
        res.push(open_tag!("head"));
        res.push(open_tag!("title"));
        res.push(self.title.clone());
        res.push(close_tag!("title"));
        res.push("<!--[if !mso]><!-- -->".into());
        res.push(open_tag!(
            "meta",
            ("http-equiv", "X-UA-Compatible"),
            ("content", "IE=edge")
        ));
        res.push("<!--<![endif]-->".into());
        res.push(open_tag!(
            "meta",
            ("http-equiv", "Content-Type"),
            ("content", "text/html; charset=UTF-8")
        ));
        res.push(open_tag!(
            "meta",
            ("name", "viewport"),
            ("content", "width=device-width, initial-scale=1")
        ));
        res.push(STYLE_BASE.into());
        res.push(close_tag!("head"));
        Ok(res.join(""))
    }
}
