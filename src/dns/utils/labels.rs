mod helper;

use crate::dns::structures::components::record_preamble::RecordPreamble;
use super::{super::structures::question::Question, bytebuffer::BytePacketBuffer};
use helper::parse_label_helper;

pub trait LabelContainer {
    fn parse_label(buffer: &mut BytePacketBuffer) -> String;
}

impl LabelContainer for Question{
    fn parse_label(buffer: &mut BytePacketBuffer) -> String {
        return parse_label_helper(buffer);
    }
}

impl LabelContainer for RecordPreamble{
    fn parse_label(buffer: &mut BytePacketBuffer) -> String {
        return parse_label_helper(buffer);
    }
}
