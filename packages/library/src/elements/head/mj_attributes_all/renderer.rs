use super::MJAttributesAll;
use crate::util::header::DefaultAttributes;

impl MJAttributesAll {
    pub fn update_attributes(&self, result: &mut DefaultAttributes) {
        result.add_all_content(self.content.iter());
    }
}
