
pub struct BytePacketBuffer {
    buffer: [u8; 512],
    pointer: usize
}

impl BytePacketBuffer{
    pub fn get_u8(self) -> Option<u8> {
        if self.pointer < self.buffer.len() {
            let temp = self.buffer[self.pointer];
            self.pointer += 1;
            Some(temp)
        }

        None
    }

    pub fn get_u16(self) -> Option<u16> {
        if self.pointer < self.buffer.len() - 1 {
            let temp: u16 = u16::from_be_bytes(self.buffer[self.pointer .. self.pointer+2]);
            self.pointer += 2;
            Some(temp)
        }

        None
    }
}
