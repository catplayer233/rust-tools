//!
//! this function was used to encode the bytes to string with the jvm specification's
//! modified utf-8 encode rule
pub fn constant_pool_utf8_decode(bytes: Vec<u8>) -> String {
    //get the target value
    let mut index = 0;
    let mut utf_8_vec: Vec<u8> = Vec::new();
    while index < bytes.len() {
        let first_byte = bytes[index];
        index += 1;
        //0x01-0x7f
        if 0x01 <= first_byte && first_byte <= 0x7F {
            utf_8_vec.push(first_byte);
            continue;
        }
        let second_byte = bytes[index];
        index += 1;
        //todo
        let code_point = u16::from_be_bytes([first_byte, second_byte]);
        if code_point == 0x0000 || (code_point >= 0x0080 && code_point <= 0x07FF) {
            utf_8_vec.push((first_byte & 0x1f) << 6 + (second_byte & 0x3f));
            continue;
        }
        if code_point >= 0x0800 && code_point <= 0xFFFF {
            //get the value
        }
    }
    String::from_utf8(utf_8_vec).unwrap()
}

#[cfg(test)]
mod test {
    use crate::constant_pool::utils::constant_pool_utf8_decode;

    #[test]
    fn build_string() {
        //CATPLAYER
        let origin_vec = vec![67, 65, 84, 80, 76, 65, 89, 69, 82];
        let target_string = constant_pool_utf8_decode(origin_vec);
        assert_eq!(target_string, String::from("CATPLAYER"));
    }
}