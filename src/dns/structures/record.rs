use std::net::{Ipv4Addr, Ipv6Addr};
use super::components::record_preamble::RecordPreamble;

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
