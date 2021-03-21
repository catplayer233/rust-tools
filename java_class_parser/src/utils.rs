pub fn get_bytes_to_u32(bytes: &mut Vec<u8>) -> u32 {
    let mut byte_array: [u8; 4] = [0; 4];
    for index in 0..4 {
        byte_array[index] = bytes.remove(0);
    }
    u32::from_be_bytes(byte_array)
}

pub fn get_bytes_to_u16(bytes: &mut Vec<u8>) -> u16 {
    let mut byte_array: [u8; 2] = [0; 2];
    for index in 0..2 {
        byte_array[index] = bytes.remove(0);
    }
    u16::from_be_bytes(byte_array)
}

