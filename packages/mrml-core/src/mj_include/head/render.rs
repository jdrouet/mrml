impl super::MjIncludeHead {
    pub(crate) fn mj_attributes_all_iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0
            .children
            .iter()
            .filter_map(|child| child.as_mj_attributes())
            .flat_map(|child| {
                child
                    .children
                    .iter()
                    .filter_map(|inner| inner.as_mj_attributes_all())
            })
            .flat_map(|child| child.attributes.iter())
            .filter_map(|(k, v)| v.as_deref().map(|inner| (k.as_str(), inner)))
    }

    pub(crate) fn mj_attributes_class_iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.0
            .children
            .iter()
            .filter_map(|child| child.as_mj_attributes())
            .flat_map(|child| {
                child
                    .children
                    .iter()
                    .filter_map(|inner| inner.as_mj_attributes_class())
            })
            .flat_map(|child| {
                child.attributes.others.iter().filter_map(move |(k, v)| {
                    v.as_deref()
                        .map(|inner| (child.attributes.name.as_str(), k.as_str(), inner))
                })
            })
    }

    pub(crate) fn mj_attributes_element_iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.0
            .children
            .iter()
            .filter_map(|child| child.as_mj_attributes())
            .flat_map(|child| {
                child
                    .children
                    .iter()
                    .filter_map(|inner| inner.as_mj_attributes_element())
            })
            .flat_map(|child| {
                child.attributes.iter().filter_map(move |(k, v)| {
                    v.as_deref()
                        .map(|inner| (child.name.as_str(), k.as_str(), inner))
                })
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
    use crate::mj_head::{MjHead, MjHeadChild};
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
            mj_head
                .children
                .push(MjHeadChild::MjBreakpoint(mj_breakpoint));
            mj_head.children.push(MjHeadChild::MjTitle(mj_title));
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
            let mj_include = MjIncludeHead::new(
                MjIncludeHeadAttributes::new("partial.mjml"),
                vec![MjIncludeHeadChild::MjTitle(mj_include_title)],
            );
            let mut mj_head = MjHead::default();
            mj_head.children.push(MjHeadChild::MjInclude(mj_include));
            mj_head
                .children
                .push(MjHeadChild::MjBreakpoint(mj_breakpoint));
            mj_head.children.push(MjHeadChild::MjTitle(mj_title));
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
            mj_head
                .children
                .push(MjHeadChild::MjBreakpoint(mj_breakpoint));
            mj_head.children.push(MjHeadChild::MjTitle(mj_title));
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
            let mj_include = MjIncludeHead::new(
                MjIncludeHeadAttributes::new("partial.mjml"),
                vec![MjIncludeHeadChild::MjTitle(mj_include_title)],
            );
            let mut mj_head = MjHead::default();
            mj_head
                .children
                .push(MjHeadChild::MjBreakpoint(mj_breakpoint));
            mj_head.children.push(MjHeadChild::MjTitle(mj_title));
            mj_head.children.push(MjHeadChild::MjInclude(mj_include));
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
            mj_head.children.push(MjHeadChild::MjTitle(MjTitle::from(
                "Hello World!".to_string(),
            )));
            mj_head.children.push(MjHeadChild::MjStyle(MjStyle::from(
                "* { background-color: red; }".to_string(),
            )));
            let mut root = Mjml::default();
            root.children.head = Some(mj_head);
            root.children.body = Some(MjBody::default());
            root.render(&opts).unwrap()
        };
        let result = {
            let opts = RenderOptions::default();
            let mj_include = MjIncludeHead::new(
                MjIncludeHeadAttributes::new("partial.mjml")
                    .with_kind(MjIncludeHeadKind::Css { inline: false }),
                vec![MjIncludeHeadChild::Text(Text::from(
                    "* { background-color: red; }".to_string(),
                ))],
            );
            let mj_head = MjHead::new(
                (),
                vec![
                    MjHeadChild::MjTitle(MjTitle::from("Hello World!".to_string())),
                    MjHeadChild::MjInclude(mj_include),
                ],
            );
            let root = Mjml::new(
                Default::default(),
                MjmlChildren {
                    head: Some(mj_head),
                    body: Some(MjBody::default()),
                },
            );
            root.render(&opts).unwrap()
        };
        similar_asserts::assert_eq!(expected, result);
    }
}
