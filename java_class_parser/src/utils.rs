use std::convert::TryInto;

use crate::constants::ClassByte;

//get the value which is big endian in java
pub fn get_bytes_to_u32(bytes: &mut Vec<ClassByte>) -> u32 {
    u32::from_be_bytes(get_class_bytes(4, bytes).try_into().unwrap())
}

pub fn get_bytes_to_i32(bytes: &mut Vec<ClassByte>) -> i32 {
    i32::from_be_bytes(get_class_bytes(4, bytes).try_into().unwrap())
}

pub fn get_bytes_to_f32(bytes: &mut Vec<ClassByte>) -> f32 {
    f32::from_be_bytes(get_class_bytes(4, bytes).try_into().unwrap())
}

pub fn get_bytes_to_i64(bytes: &mut Vec<ClassByte>) -> i64 {
    i64::from_be_bytes(get_class_bytes(8, bytes).try_into().unwrap())
}

pub fn get_bytes_to_f64(bytes: &mut Vec<ClassByte>) -> f64 {
    f64::from_be_bytes(get_class_bytes(8, bytes).try_into().unwrap())
}

pub fn get_bytes_to_u16(bytes: &mut Vec<ClassByte>) -> u16 {
    u16::from_be_bytes(get_class_bytes(2, bytes).try_into().unwrap())
}

pub fn get_class_byte(class_bytes: &mut Vec<ClassByte>) -> ClassByte {
    class_bytes.remove(0)
}

pub fn get_class_bytes(size: usize, class_bytes: &mut Vec<ClassByte>) -> Vec<ClassByte> {
    //operate the vec directly by ptr
    let mut vec = Vec::with_capacity(size);
    for target_byte in class_bytes.drain(0..size) {
        vec.push(target_byte);
    }
    vec
}

#[cfg(test)]
mod test {
    use crate::constants::ClassByte;
    use crate::utils::{get_bytes_to_i64, get_bytes_to_u32, get_class_bytes};

    #[test]
    fn test_long_convert() {
        let mut test_class_bytes: Vec<ClassByte> = vec![0, 0, 0, 1, 0, 0, 0, 1];
        let bytes_to_long = get_bytes_to_i64(&mut test_class_bytes);
        assert_eq!(bytes_to_long.to_be_bytes(), [0, 0, 0, 1, 0, 0, 0, 1])
    }

    #[test]
    fn get_bytes() {
        let mut origin_vec: Vec<u8> = vec![1, 2, 3];
        let sub_vec = get_class_bytes(2, &mut origin_vec);
        assert_eq!(sub_vec, vec![1, 2]);
        assert_eq!(origin_vec, vec![3]);
    }
}

