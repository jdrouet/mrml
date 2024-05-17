impl super::MjIncludeHead {
    pub(crate) fn mj_attributes_all_iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.children
            .iter()
            .filter_map(|child| child.as_mj_attributes())
            .flat_map(|child| {
                child
                    .children
                    .iter()
                    .filter_map(|inner| inner.as_mj_attributes_all())
            })
            .flat_map(|child| child.attributes.iter())
            .map(|(k, v)| (k.as_str(), v.as_str()))
    }

    pub(crate) fn mj_attributes_class_iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.children
            .iter()
            .filter_map(|child| child.as_mj_attributes())
            .flat_map(|child| {
                child
                    .children
                    .iter()
                    .filter_map(|inner| inner.as_mj_attributes_class())
            })
            .flat_map(|child| {
                child
                    .attributes
                    .iter()
                    .map(move |(k, v)| (child.name.as_str(), k.as_str(), v.as_str()))
            })
    }

    pub(crate) fn mj_attributes_element_iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.children
            .iter()
            .filter_map(|child| child.as_mj_attributes())
            .flat_map(|child| {
                child
                    .children
                    .iter()
                    .filter_map(|inner| inner.as_mj_attributes_element())
            })
            .flat_map(|child| {
                child
                    .attributes
                    .iter()
                    .map(move |(k, v)| (child.name.as_str(), k.as_str(), v.as_str()))
            })
    }
}

impl super::MjIncludeHeadKind {
    #[inline]
    pub(crate) fn is_mjml(&self) -> bool {
        matches!(self, Self::Mjml)
    }

    #[inline]
    pub(crate) fn is_css(&self, value: bool) -> bool {
        match self {
            Self::Css { inline } => *inline == value,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_body::MjBody;
    use crate::mj_breakpoint::MjBreakpoint;
    use crate::mj_head::MjHead;
    use crate::mj_include::head::{
        MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild, MjIncludeHeadKind,
    };
    use crate::mj_style::MjStyle;
    use crate::mj_title::MjTitle;
    use crate::mjml::{Mjml, MjmlChildren};
    use crate::prelude::render::RenderOptions;
    use crate::text::Text;

    #[test]
    fn basic_mjml_kind_include_first() {
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
            mj_include.attributes.path = "partial.mjml".to_string();
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
            let mj_include = MjIncludeHead {
                attributes: MjIncludeHeadAttributes {
                    path: "partial.mjml".into(),
                    kind: Default::default(),
                },
                children: vec![MjIncludeHeadChild::MjTitle(mj_include_title)],
            };
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
            let mj_include = MjIncludeHead {
                attributes: MjIncludeHeadAttributes {
                    path: "partial.mjml".to_owned(),
                    kind: MjIncludeHeadKind::Css { inline: false },
                },
                children: vec![MjIncludeHeadChild::Text(Text::from(
                    "* { background-color: red; }".to_string(),
                ))],
            };
            let mj_head = MjHead {
                children: vec![
                    MjTitle::from("Hello World!".to_string()).into(),
                    mj_include.into(),
                ],
            };
            let root = Mjml {
                attributes: Default::default(),
                children: MjmlChildren {
                    head: Some(mj_head),
                    body: Some(MjBody::default()),
                },
            };
            root.render(&opts).unwrap()
        };
        similar_asserts::assert_eq!(expected, result);
    }
}
