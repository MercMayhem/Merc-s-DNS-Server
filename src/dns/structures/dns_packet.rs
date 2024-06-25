use crate::dns::structures::header::Header;

struct DnsPacket{
    header: Header,
    question: Vec<Question>,
    answer: Vec<Answer>,
    authority: Vec<Authority>,
    additional: Vec<Additional>
}
