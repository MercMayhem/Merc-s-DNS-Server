use std::error::Error;
use super::bytebuffer::BytePacketBuffer;

pub trait BufParse: Sized{
    fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<Self, Box<dyn Error>>;
}

