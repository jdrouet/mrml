use super::MJAttributesClass;
use crate::util::header::DefaultAttributes;

impl MJAttributesClass {
    pub fn update_attributes(&self, result: &mut DefaultAttributes) {
        result.add_class_content(self.name.as_str(), self.content.iter());
    }
}
