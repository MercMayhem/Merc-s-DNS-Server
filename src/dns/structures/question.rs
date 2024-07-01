use std::error::Error;
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
            curr += label[curr] as usize + 1;
        }

        if curr == label.len(){
            return true
        }

        false
    }

    fn parse_question(buffer: &mut BytePacketBuffer) -> Question{
        
        todo!()
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
            println!("{check}, {check2}");
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
            println!("{check}, {check2}");
            assert_eq!(check2, false)
        }
    }
}
