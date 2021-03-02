use super::MJMLElement;
use crate::elements::error::Error;
use crate::elements::prelude::Component;

impl<'a> MJMLElement {
    pub fn get_title(&self) -> String {
        self.head.get_title()
    }

    pub fn get_preview(&self) -> String {
        self.head.get_preview()
    }

    pub fn get_html(&self) -> Result<String, Error> {
        let header = self.head.get_header();
        Ok(String::from("<!doctype html>")
           + "<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">"
           + self.head.render(&header)?.as_str()
           + self.body.render(&header)?.as_str()
           + "</html>")
    }
}
