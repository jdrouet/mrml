use super::MJAttributesElement;
use crate::util::header::DefaultAttributes;

impl MJAttributesElement {
    pub fn update_attributes(&self, result: &mut DefaultAttributes) {
        result.add_element_content(self.name.as_str(), self.content.iter());
    }
}
