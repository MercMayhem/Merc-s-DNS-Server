#[derive(Debug)]
pub enum ResponseCode{
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResponseCode {
    pub fn from_num(num: u8) -> ResponseCode{
        match num {
            1 => Self::FORMERR,
            2 => Self::SERVFAIL,
            3 => Self::NXDOMAIN,
            4 => Self::NOTIMP,
            5 => Self::REFUSED,
            0 | _ => Self::NOERROR
        }
    }
}
