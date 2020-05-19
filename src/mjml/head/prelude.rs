use crate::util::Header;

pub trait HeadComponent {
    fn update_header(&self, header: &mut Header);
}
