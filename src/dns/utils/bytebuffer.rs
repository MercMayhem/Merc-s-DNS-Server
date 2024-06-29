#[derive(Debug)]
pub struct BytePacketBuffer {
    buffer: [u8; 512],
    pointer: usize
}

impl BytePacketBuffer{
    pub fn get_u8(&mut self) -> Option<u8> {
        if self.pointer < self.buffer.len() {
            let temp = self.buffer[self.pointer];
            self.pointer += 1;
            Some(temp);
        }

        None
    }

    pub fn get_u16(&mut self) -> Option<u16> {
        if self.pointer < self.buffer.len() - 1 {
            let temp: u16 = u16::from_be_bytes([self.buffer[self.pointer], self.buffer[self.pointer+1]]);
            self.pointer += 2;
            Some(temp);
        }

        None
    }

    pub fn get(&mut self, num: usize) -> Option<&[u8]>{
        if self.pointer + num < self.buffer.len() {
            let temp = &self.buffer[self.pointer .. self.pointer+num];
            self.pointer += num;
            return Some(temp);
        }

        None
    }

    pub fn new(bytes: &Vec<u8>) -> Result<BytePacketBuffer, String>{
        if bytes.len() > 512 {
            return Err("Packet Too Big".to_string());
        }
        
        let mut buffer:[u8; 512] = [0; 512];
        let mut pos = 0;
        for byte in bytes.iter(){
            buffer[pos] = *byte;
            pos += 1
        }

        return Ok(BytePacketBuffer { buffer, pointer: 0 })
    }
}
