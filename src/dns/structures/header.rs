use crate::dns::structures::components::response_code::ResponseCode;

pub struct Header{
    pub id: u16, // 16 bits

    pub query_response: bool, // 1 bit
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
