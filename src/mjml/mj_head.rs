use super::{Component, Error};
use crate::util::{Context, Header};
use crate::{close_tag, open_tag, to_attributes};
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
    context: Option<Context>,
    header: Header,
    node: Option<Node<'a, 'b>>,
}

impl MJHead<'_, '_> {
    pub fn empty<'a, 'b>() -> MJHead<'a, 'b> {
        debug!("create empty");
        MJHead {
            context: None,
            header: Header::new(),
            node: None,
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJHead<'a, 'b>, Error> {
        debug!("parse");
        Ok(MJHead {
            context: None,
            header: Header::new(),
            node: Some(node),
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
        let breakpoint = "480px";
        if !self.header.has_media_queries() {
            return "".into();
        }
        let mut res = vec![];
        res.push(open_tag!("style", to_attributes!(("type", "text/css"))));
        res.push(format!("@media only screen and (min-width:{}) {{ ", breakpoint));
        for (classname, size) in self.header.get_media_queries().iter() {
            res.push(format!(".{} {{ width:{} !important; max-width: {}; }}", classname, size.to_string(), size.to_string()));
        }
        res.push("}".into());
        res.push(close_tag!("style"));
        res.join("\n")
    }
}

impl Component for MJHead<'_, '_> {
    fn node(&self) -> Option<Node> {
        self.node
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
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
        res.push(self.get_media_queries());
        res.push(close_tag!("head"));
        Ok(res.join(""))
    }
}
