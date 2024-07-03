use super::BytePacketBuffer;

pub fn parse_label_helper(buffer: &mut BytePacketBuffer) -> String {
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
            component = parse_label_from_ptr(buffer, ptr).into();
            curr += 2;
        }

        ret.push_str(&String::from_utf8(component).unwrap());
        ret.push('.');
    }

    return ret[..ret.len()-1].to_string();
}

fn parse_label_from_ptr(buffer: &BytePacketBuffer, ptr: usize) -> String {
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
            
            component = parse_label_from_ptr(buffer, ptr).into();

            curr += 2;
        }
        ret.push_str(&String::from_utf8(component).unwrap());
        ret.push('.');

    }

    return ret[..ret.len()-1].to_string();
}
