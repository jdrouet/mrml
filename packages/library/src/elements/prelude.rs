use crate::elements::error::Error;
use crate::util::context::Context;
use crate::util::header::Header;

pub trait Component {
    fn context(&self) -> Option<&Context>;
    fn set_context(&mut self, ctx: Context);

    fn update_header(&self, _header: &mut Header) {}
    fn render(&self, header: &Header) -> Result<String, Error>;
}
