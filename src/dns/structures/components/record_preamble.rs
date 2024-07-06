use std::error::Error;
use crate::dns::utils::{labels::LabelContainer, bytebuffer::BytePacketBuffer};

#[derive(Debug)]
pub struct RecordPreamble{
    pub name: String,
    pub r_type: u16,
    pub ttl: u32,
    pub len: u16
}

impl RecordPreamble {
    fn valid_size(buffer: &BytePacketBuffer) -> bool{
        match buffer.find_byte_pos(0){
            Some(pos) => {
                if pos + 10 < buffer.len(){
                    return true;
                }

                return false;
            },
            None => return false
        }
    }

    fn valid_label(buffer: &BytePacketBuffer) -> bool{
        let end = buffer.find_byte_pos(0).unwrap();
        let label = buffer.get(end - buffer.get_pointer()).unwrap();
        
        let mut curr = 0;
        while curr < label.len(){
            curr += if label[curr] & 0xF0 != 0xC0 {
                        label[curr] as usize + 1
                    } else {
                        2
                    }
        }

        if curr == label.len(){
            return true
        }

        false
    }

    fn parse_r_type(buffer: &mut BytePacketBuffer) -> u16{
        return buffer.get_mut_u16().unwrap()
    }

    fn parse_ttl(buffer: &mut BytePacketBuffer) -> u32{
        let byte1 = buffer.get_mut_u16().unwrap();
        let byte2 = buffer.get_mut_u16().unwrap();
        return ((byte1 as u32) << 16) | (byte2 as u32)
    }

    fn parse_len(buffer: &mut BytePacketBuffer) -> u16{
        return buffer.get_mut_u16().unwrap()
    }

    fn parse_preamble(buffer: &mut BytePacketBuffer) -> RecordPreamble{
        let name = RecordPreamble::parse_label(buffer);
        let r_type = RecordPreamble::parse_r_type(buffer);

        let _ = buffer.get_mut_u16();

        let ttl = RecordPreamble::parse_ttl(buffer);
        let len = RecordPreamble::parse_len(buffer);

        RecordPreamble {name, r_type, ttl, len}
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<RecordPreamble, Box<dyn Error>>{
        if !RecordPreamble::valid_size(buffer){
            return Err("Wrong Size".to_string().into())
        }

        if !RecordPreamble::valid_label(buffer){
            return Err("Incorrect label".to_string().into())
        }

        let preamble = RecordPreamble::parse_preamble(buffer);
        Ok(preamble)
    }
}

#[cfg(test)]
mod tests{
    use super::{BytePacketBuffer, RecordPreamble};
    use std::fs;

    #[test]
    fn test_record_preamble_parsing(){
        let query = fs::read("tests/response_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let _ = buffer.get_mut(28).unwrap();
        let preamble = RecordPreamble::parse_preamble(&mut buffer);

        println!("{:?}", preamble);
    }
}
