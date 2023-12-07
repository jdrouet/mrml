#[cfg(test)]
mod tests {
    use crate::mj_body::MjBody;
    use crate::mj_breakpoint::MjBreakpoint;
    use crate::mj_head::MjHead;
    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadChild, MjIncludeHeadKind};
    use crate::mj_style::MjStyle;
    use crate::mj_title::MjTitle;
    use crate::mjml::Mjml;
    use crate::prelude::render::RenderOptions;
    use crate::text::Text;

    #[test]
    fn basic_mjml_kind_include_first() {
        let expected = {
            let opts = RenderOptions::default();
            let mut mj_breakpoint = MjBreakpoint::default();
            mj_breakpoint.attributes.width = "500px".into();
            let mj_title = MjTitle::from("Hello New World!".to_string());
            let mut mj_head = MjHead::default();
            mj_head.children.push(mj_breakpoint.into());
            mj_head.children.push(mj_title.into());
            let mut root = Mjml::default();
            root.children.head = Some(mj_head);
            root.children.body = Some(MjBody::default());
            root.render(&opts).unwrap()
        };
        let result = {
            let opts = RenderOptions::default();
            let mut mj_breakpoint = MjBreakpoint::default();
            mj_breakpoint.attributes.width = "500px".into();
            let mj_title = MjTitle::from("Hello Old World!".to_string());
            let mj_include_title = MjTitle::from("Hello New World!".to_string());
            let mut mj_include = MjIncludeHead::default();
            mj_include.attributes.path = "partial.mjml".to_owned();
            mj_include
                .children
                .push(MjIncludeHeadChild::MjTitle(mj_include_title));
            let mut mj_head = MjHead::default();
            mj_head.children.push(mj_include.into());
            mj_head.children.push(mj_breakpoint.into());
            mj_head.children.push(mj_title.into());
            let mut root = Mjml::default();
            root.children.head = Some(mj_head);
            root.children.body = Some(MjBody::default());
            root.render(&opts).unwrap()
        };
        similar_asserts::assert_eq!(expected, result);
    }

    #[test]
    fn basic_mjml_kind_include_last() {
        let expected = {
            let opts = RenderOptions::default();
            let mut mj_breakpoint = MjBreakpoint::default();
            mj_breakpoint.attributes.width = "500px".into();
            let mj_title = MjTitle::from("Hello Old World!".to_string());
            let mut mj_head = MjHead::default();
            mj_head.children.push(mj_breakpoint.into());
            mj_head.children.push(mj_title.into());
            let mut root = Mjml::default();
            root.children.head = Some(mj_head);
            root.children.body = Some(MjBody::default());
            root.render(&opts).unwrap()
        };
        let result = {
            let opts = RenderOptions::default();
            let mut mj_breakpoint = MjBreakpoint::default();
            mj_breakpoint.attributes.width = "500px".into();
            let mj_title = MjTitle::from("Hello Old World!".to_string());
            let mj_include_title = MjTitle::from("Hello New World!".to_string());
            let mut mj_include = MjIncludeHead::default();
            mj_include.attributes.path = "partial.mjml".to_owned();
            mj_include
                .children
                .push(MjIncludeHeadChild::MjTitle(mj_include_title));
            let mut mj_head = MjHead::default();
            mj_head.children.push(mj_breakpoint.into());
            mj_head.children.push(mj_title.into());
            mj_head.children.push(mj_include.into());
            let mut root = Mjml::default();
            root.children.head = Some(mj_head);
            root.children.body = Some(MjBody::default());
            root.render(&opts).unwrap()
        };
        similar_asserts::assert_eq!(expected, result);
    }

    #[test]
    fn css_kind() {
        let expected = {
            let opts = RenderOptions::default();
            let mut mj_head = MjHead::default();
            mj_head
                .children
                .push(MjTitle::from("Hello World!".to_string()).into());
            mj_head
                .children
                .push(MjStyle::from("* { background-color: red; }".to_string()).into());
            let mut root = Mjml::default();
            root.children.head = Some(mj_head);
            root.children.body = Some(MjBody::default());
            root.render(&opts).unwrap()
        };
        let result = {
            let opts = RenderOptions::default();
            let mut mj_include = MjIncludeHead::default();
            mj_include.attributes.path = "partial.mjml".to_owned();
            mj_include.attributes.kind = MjIncludeHeadKind::Css { inline: false };
            mj_include
                .children
                .push(Text::from("* { background-color: red; }".to_string()).into());
            let mut mj_head = MjHead::default();
            mj_head
                .children
                .push(MjTitle::from("Hello World!".to_string()).into());
            mj_head.children.push(mj_include.into());
            let mut root = Mjml::default();
            root.children.head = Some(mj_head);
            root.children.body = Some(MjBody::default());
            root.render(&opts).unwrap()
        };
        similar_asserts::assert_eq!(expected, result);
    }
}
