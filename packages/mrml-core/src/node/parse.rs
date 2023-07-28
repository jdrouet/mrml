// impl<T> Parser for NodeParser<T>
// where
//     T: Parsable,
//     T: From<Comment>,
//     T: From<Text>,
// {
//     type Output = Node<T>;

//     fn build(self) -> Result<Self::Output, Error> {
//         Ok(Node {
//             tag: self.tag,
//             attributes: self.attributes,
//             children: self.children,
//         })
//     }

//     fn should_ignore_children(&self) -> bool {
//         matches!(
//             self.tag.as_str(),
//             "area"
//                 | "base"
//                 | "br"
//                 | "col"
//                 | "embed"
//                 | "hr"
//                 | "img"
//                 | "input"
//                 | "link"
//                 | "meta"
//                 | "param"
//                 | "source"
//                 | "track"
//                 | "wbr"
//         )
//     }

//     parse_attribute!();

//     fn parse_child_element<'a>(
//         &mut self,
//         tag: xmlparser::StrSpan<'a>,
//         tokenizer: &mut xmlparser::Tokenizer<'a>,
//     ) -> Result<(), Error> {
//         self.children
//             .push(T::parse(tag, tokenizer, self.opts.clone())?);
//         Ok(())
//     }

//     parse_comment!();
//     parse_text!();
// }
