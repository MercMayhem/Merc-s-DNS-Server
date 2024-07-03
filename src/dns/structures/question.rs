use std::{error::Error, iter};
use crate::dns::utils::bytebuffer::BytePacketBuffer;

#[derive(Debug)]
pub struct Question{
    pub name: String,
    pub r_type: u16,
}

impl Question {
    fn valid_size(buffer: &BytePacketBuffer) -> bool{
        match buffer.find_byte_pos(0){
            Some(pos) => {
                if pos + 4 < buffer.len(){
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

    fn parse_label_from_ptr(buffer: &BytePacketBuffer, ptr: usize) -> String{
        let end = buffer.find_byte_pos_after_ptr(ptr, 0).unwrap();
        let label = buffer.get_from_ptr(ptr, end - ptr);

        let mut ret = String::new();

        let mut curr = 0;
        while curr < label.len(){
            let mut component: Vec<u8> = Vec::new();
            if label[curr] & 0xF0 != 0xC0 {
                for ch in label[curr+1..=curr+label[curr] as usize].iter(){
                    component.push(*ch);
                }

                curr += label[curr] as usize + 1;
            } else {
                let ptr = label[curr + 1] as usize;
                
                component = Question::parse_label_from_ptr(buffer, ptr).into();

                curr += 2;
            }
            ret.push_str(&String::from_utf8(component).unwrap());
            ret.push('.');

        }

        return ret[..ret.len()-1].to_string();
    }

    fn parse_label(buffer: &mut BytePacketBuffer) -> String{
        let end = buffer.find_byte_pos(0).unwrap();
        let label = buffer.get_mut(end - buffer.get_pointer()).unwrap();

        let mut ret = String::new();

        let mut curr = 0;
        while curr < label.len(){
            let mut component: Vec<u8> = Vec::new();
            if label[curr] & 0xF0 != 0xC0 {
                for ch in label[curr+1..=curr+label[curr] as usize].iter(){
                    component.push(*ch);
                }

                curr += label[curr] as usize + 1;
            } else {
                let ptr = label[curr + 1] as usize;
                component = Question::parse_label_from_ptr(buffer, ptr).into();
                curr += 2;
            }

            ret.push_str(&String::from_utf8(component).unwrap());
            ret.push('.');
        }

        return ret[..ret.len()-1].to_string();
    }

    fn parse_r_type(buffer: &mut BytePacketBuffer) -> u16{
        return buffer.get_mut_u16().unwrap();
    }

    fn parse_question(buffer: &mut BytePacketBuffer) -> Question{
        let name: String = Question::parse_label(buffer);
        let r_type: u16 = Question::parse_r_type(buffer);

        todo!();
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<Question, Box<dyn Error>>{
        if !Question::valid_size(buffer){
            return Err("Wrong Size".to_string().into())
        }

        if !Question::valid_label(buffer){
            return Err("Incorrect label".to_string().into())
        }

        let question = Question::parse_question(buffer);
        Ok(question)
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::fs;

    #[test]
    fn check_valid_length(){
        let query = fs::read("tests/query_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let _ = buffer.get_mut(12).unwrap();
        let check = Question::valid_size(&buffer);

        assert_eq!(check, true)
    }

    #[test]
    fn check_valid_label(){
        let query = fs::read("tests/query_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let _ = buffer.get_mut(12).unwrap();
        let check = Question::valid_size(&buffer);

        if check {
            let check2 = Question::valid_label(&buffer);
            // println!("{check}, {check2}");
            assert_eq!(check2, true)
        }
    }


    #[test]
    fn check_valid_label2(){
        let query = fs::read("tests/malformed_query_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let _ = buffer.get_mut(12).unwrap();
        let check = Question::valid_size(&buffer);

        if check {
            let check2 = Question::valid_label(&buffer);
            // println!("{check}, {check2}");
            assert_eq!(check2, false)
        }
    }

    #[test]
    fn check_label_parsing(){
        let query = fs::read("tests/query_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let _ = buffer.get_mut(12).unwrap();
        let label = Question::parse_label(&mut buffer);
        
        // println!("{}", label)
        assert_eq!("google.com".to_string(), label)
    }

    #[test]
    fn check_label_parsing2(){
        let query = fs::read("tests/response_packet.txt").unwrap();
        let mut buffer = BytePacketBuffer::new(&query).unwrap();

        let _ = buffer.get_mut(28).unwrap();
        let label = Question::parse_label(&mut buffer);
        
        // println!("{}", label)
        assert_eq!("google.com".to_string(), label)
    }
}
