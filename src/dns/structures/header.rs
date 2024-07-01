use std::error::Error;
use crate::dns::structures::components::response_code::ResponseCode;
use crate::dns::utils::bytebuffer::BytePacketBuffer;

#[derive(Debug)]
pub struct Header{
    pub id: u16, // 16 bits

    pub query: bool, // 1 bit
    pub response: bool,
    pub opcode: u8,
    pub authoritative_answer: bool, // 1 bit
    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,

    pub response_code: ResponseCode,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16
}

impl Header {
    fn parse_header(bytes: &[u8]) -> Header{
        let id = u16::from_be_bytes([bytes[0], bytes[1]]);

        let second_part: u16 = u16::from_be_bytes([bytes[2], bytes[3]]);

        let query: bool = (second_part >> 15) & 0b1 == 0;
        let response: bool = !query;
        let opcode: u8 = ((second_part >> 11) & 0b1111).try_into().unwrap();
        let authoritative_answer: bool = (second_part >> 10) & 0b1 == 1;
        let truncated_message: bool = (second_part >> 9) & 0b1 == 1;
        let recursion_desired: bool = (second_part >> 8) & 0b1 == 1;
        let recursion_available: bool = (second_part >> 7) & 0b1 == 1;
        let response_code: ResponseCode = {
            let response_num: u8 = (second_part & 0b1111).try_into().unwrap();
            ResponseCode::from_num(response_num)
        };

        let question_count: u16 = u16::from_be_bytes([bytes[4], bytes[5]]);
        let answer_count: u16 = u16::from_be_bytes([bytes[6], bytes[7]]);
        let authority_count: u16 = u16::from_be_bytes([bytes[8], bytes[9]]);
        let additional_count: u16 = u16::from_be_bytes([bytes[10], bytes[11]]);

        return Header{
            id,
            query,
            response,
            opcode,
            authoritative_answer,
            truncated_message,
            recursion_desired,
            recursion_available,
            response_code,
            question_count,
            answer_count,
            authority_count,
            additional_count
        }
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<Header, Box<dyn Error>>{
        let header_bytes = buffer.get_mut(12).unwrap();
        if header_bytes.len() != 12 {
            return Err("Not enough bytes for header".to_string().into())
        }

        Ok(Header::parse_header(header_bytes))
    }


}

#[cfg(test)]
mod tests{
    use super::*;
    use std::fs;

    #[test]
    fn check_header_parse() {
        let query = fs::read("tests/query_packet.txt").unwrap();
        let mut buf = BytePacketBuffer::new(&query).unwrap();
        let header = Header::from_buffer(&mut buf).unwrap();

        println!("{:#?}", header)
    }

    #[test]
    fn check_header_parse2() {
        let query = fs::read("tests/response_packet.txt").unwrap();
        let mut buf = BytePacketBuffer::new(&query).unwrap();
        let header = Header::from_buffer(&mut buf).unwrap();

        println!("{:#?}", header)
    }
}
