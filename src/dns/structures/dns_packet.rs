use crate::dns::structures::header::Header;
use crate::dns::structures::question::Question;
use crate::dns::structures::record::Record;

struct DnsPacket{
    header: Header,
    question: Vec<Question>,
    answer: Vec<Record>,
    authority: Vec<Record>,
    additional: Vec<Record>
}
