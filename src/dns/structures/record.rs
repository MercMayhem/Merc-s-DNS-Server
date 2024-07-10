use std::{net::{Ipv4Addr, Ipv6Addr}, error::Error};
use crate::dns::utils::bytebuffer::BytePacketBuffer;
use super::components::record_preamble::RecordPreamble;
use crate::dns::utils::labels::LabelContainer;

#[derive(Debug)]
pub enum Record{
    A {
        preamble: RecordPreamble,
        ipaddr: Ipv4Addr   
    },

    NS {
        preamble: RecordPreamble,
        host: String
    },

    CNAME {
        preamble: RecordPreamble,
        host: String
    },

    MX {
        preamble: RecordPreamble,
        priority: u16,
        host: String
    },

    AAAA{
        preamble: RecordPreamble,
        ipv6addr: Ipv6Addr
    }
}

impl Record {
    fn valid_size(buffer: &BytePacketBuffer) -> bool{
        let ptr = buffer.get_pointer();
        if ptr + 4 <= buffer.len(){
            return true
        }
        false
    }

    fn parse_ipv4addr(buffer: &mut BytePacketBuffer) -> Option<Ipv4Addr>{
        let oc1 = buffer.get_mut_u8();
        let oc2 = buffer.get_mut_u8();
        let oc3 = buffer.get_mut_u8();
        let oc4 = buffer.get_mut_u8();

        if oc1.is_some() && oc2.is_some() && oc3.is_some() && oc4.is_some(){
            return Some(Ipv4Addr::new(oc1.unwrap(), oc2.unwrap(), oc3.unwrap(), oc3.unwrap()))
        }

        None
    }

    fn valid_label_size(buffer: &BytePacketBuffer) -> bool{
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

    fn parse_ipv6addr(buffer: &mut BytePacketBuffer) -> Option<Ipv6Addr>{
        let oc1 = buffer.get_mut_u16();
        let oc2 = buffer.get_mut_u16();
        let oc3 = buffer.get_mut_u16();
        let oc4 = buffer.get_mut_u16();
        let oc5 = buffer.get_mut_u16();
        let oc6 = buffer.get_mut_u16();
        let oc7 = buffer.get_mut_u16();
        let oc8 = buffer.get_mut_u16();

        if oc1.is_some() && oc2.is_some() && oc3.is_some() && oc4.is_some() && oc5.is_some() && oc6.is_some() && oc7.is_some() && oc8.is_some(){
            return Some(Ipv6Addr::new(oc1.unwrap(), oc2.unwrap(), oc3.unwrap(), oc4.unwrap(), oc5.unwrap(), oc6.unwrap(), oc7.unwrap(), oc8.unwrap()))
        }

        None
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<Record, Box<dyn Error>>{
        let preamble = RecordPreamble::from_buffer(buffer)?;

        if !Record::valid_size(buffer){
            return Err("Incorrect Size".to_string().into()) 
        }

        match preamble.r_type{
            1 => {
                let ipaddr = Record::parse_ipv4addr(buffer);
                match ipaddr {
                    Some(addr) => return Ok(Record::A { preamble, ipaddr: addr }),
                    None => return Err("Not enough bytes".to_string().into())
                }
            }, // A
            2 | 5 => {
                if !Record::valid_label_size(buffer){
                    return Err("Wrong Size".to_string().into())
                }

                if !Record::valid_label(buffer){
                    return Err("Incorrect label".to_string().into())
                }

                let host = Record::parse_label(buffer);
                if preamble.r_type == 2{
                    return Ok(Record::NS { preamble, host })
                } else {
                    return Ok(Record::CNAME { preamble, host })
                }
            }, // NS and CNAME
            15 => {
                let priority_option = buffer.get_mut_u16();
                let priority: u16;

                if priority_option.is_some(){
                    priority = priority_option.unwrap()
                } else {
                    return Err("Not enough bytes".to_string().into())   
                }

                if !Record::valid_label_size(buffer){
                    return Err("Wrong Size".to_string().into())
                }

                if !Record::valid_label(buffer){
                    return Err("Incorrect label".to_string().into())
                }

                let host = Record::parse_label(buffer);
                
                return Ok(Record::MX { preamble, priority, host })
            }, // MX
            28 => {
                let ipaddr = Record::parse_ipv6addr(buffer);
                match ipaddr {
                    Some(addr) => return Ok(Record::AAAA { preamble, ipv6addr: addr }),
                    None => return Err("Not enough bytes".to_string().into())
                }
            }, // AAAA
            _ => {
                return Err("Incorrect or Unrecognized Type".to_string().into())
            },
        }
    }
}

#[cfg(test)]
mod tests{
    use std::fs;
    use crate::dns::utils::bytebuffer::BytePacketBuffer;
    use super::Record;

    #[test]
    fn check_a_record_parsing(){
        let query = fs::read("tests/response_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let _ = buffer.get_mut(28).unwrap();
        let record = Record::from_buffer(&mut buffer);

        println!("{:?}", record)
    }
}
