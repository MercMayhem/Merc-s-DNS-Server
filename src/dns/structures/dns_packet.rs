use std::error::Error;
use crate::dns::structures::header::Header;
use crate::dns::structures::question::Question;
use crate::dns::structures::record::Record;
use crate::dns::utils::bufparse::BufParse;
use crate::dns::utils::bytebuffer::BytePacketBuffer;

#[derive(Debug)]
struct DnsPacket{
    header: Header,
    question: Vec<Question>,
    answer: Vec<Record>,
    authority: Vec<Record>,
    additional: Vec<Record>
}

impl DnsPacket{

    fn parse_questions_or_records<T: BufParse>(buffer: &mut BytePacketBuffer, count: u16) -> Result<Vec<T>, Box<dyn Error>>{
        let mut ret: Vec<T> = Vec::new();

        for _ in 0..count{
            let entry = T::from_buffer(buffer)?;
            ret.push(entry)
        }

        Ok(ret)
    }
    
    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket, Box<dyn Error>> {
        let header = Header::from_buffer(buffer)?;

        let question: Vec<Question> = DnsPacket::parse_questions_or_records(buffer, header.question_count)?;
        let answer: Vec<Record> = DnsPacket::parse_questions_or_records(buffer, header.answer_count)?;
        let authority: Vec<Record> = DnsPacket::parse_questions_or_records(buffer, header.authority_count)?;
        let additional: Vec<Record> = DnsPacket::parse_questions_or_records(buffer, header.additional_count)?;
        
        Ok(DnsPacket { header, question, answer, authority, additional })
    }
}


#[cfg(test)]
mod tests{
    use std::fs;
    use crate::dns::utils::bytebuffer::BytePacketBuffer;
    use super::DnsPacket;
    
    #[test]
    fn check_dns_packet_parsing() {
        let query = fs::read("tests/query_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let packet = DnsPacket::from_buffer(&mut buffer).unwrap();
        println!("{:?}", packet)
    }

    #[test]
    fn check_dns_packet_parsing2() {
        let query = fs::read("tests/response_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let packet = DnsPacket::from_buffer(&mut buffer).unwrap();
        println!("{:?}", packet)
    }
}
