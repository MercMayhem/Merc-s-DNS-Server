#[derive(Debug)]
pub struct BytePacketBuffer {
    buffer: [u8; 512],
    pointer: usize
}

impl BytePacketBuffer{
    pub fn get_mut_u8(&mut self) -> Option<u8> {
        if self.pointer < self.buffer.len() {
            let temp = self.buffer[self.pointer];
            self.pointer += 1;
            Some(temp);
        }

        None
    }

    pub fn get_mut_u16(&mut self) -> Option<u16> {
        if self.pointer < self.buffer.len() - 1 {
            let temp: u16 = u16::from_be_bytes([self.buffer[self.pointer], self.buffer[self.pointer+1]]);
            self.pointer += 2;
            Some(temp);
        }

        None
    }

    pub fn get_mut(&mut self, num: usize) -> Option<Vec<u8>>{
        if self.pointer + num < self.buffer.len() {
            let temp = &self.buffer[self.pointer .. self.pointer+num];
            self.pointer += num;
            return Some(temp.to_vec());
        }

        None
    }

    pub fn get(&self, num: usize) -> Option<&[u8]>{
        if self.pointer + num < self.buffer.len() {
            let temp = &self.buffer[self.pointer .. self.pointer+num];
            return Some(temp);
        }

        None
    }

    pub fn get_from_ptr(&self, ptr: usize, num: usize) -> &[u8]{
        &self.buffer[ptr .. ptr + num]
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

        Ok(BytePacketBuffer { buffer, pointer: 0 })
    }

    pub fn find_byte_pos(&self, target: u8) -> Option<usize>{
        for (i, byte) in self.buffer[self.pointer..].iter().enumerate(){
            if *byte == target{
                return Some(self.pointer+i)
            }
        } 
        None
    }

    pub fn find_byte_pos_after_ptr(&self, ptr: usize, target: u8) -> Option<usize>{
        for (i, byte) in self.buffer[ptr..].iter().enumerate(){
            if *byte == target{
                return Some(ptr+i)
            }
        }

        None
    }

    pub fn len(&self) -> usize{
        self.buffer.len()
    }

    pub fn get_pointer(&self) -> usize{
        self.pointer
    }
}
