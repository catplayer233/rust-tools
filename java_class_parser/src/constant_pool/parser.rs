use crate::constant_pool::constants::*;
use crate::constant_pool::utils::constant_pool_utf8_decode;
use crate::constants::ClassByte;
use crate::utils::{get_bytes_to_f32, get_bytes_to_f64, get_bytes_to_i32, get_bytes_to_i64, get_bytes_to_u16, get_bytes_to_u32, get_class_byte, get_class_bytes};

enum ConstantPoolEntry {
    ClassInfo {
        tag: u8,
        name_index: u16,
    },
    RefInfo {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16,
    },
    StringInfo {
        tag: u8,
        string_index: u16,
    },
    IntegerInfo {
        tag: u8,
        value: i32,
    },
    FloatInfo {
        tag: u8,
        value: f32,
    },
    LongInfo {
        tag: u8,
        value: i64,
    },
    DoubleInfo {
        tag: u8,
        value: f64,
    },
    NameAndTypeInfo {
        tag: u8,
        name_index: u16,
        descriptor_index: u16,
    },
    UTF8Info {
        tag: u8,
        content: String,
    },
    MethodHandleInfo {
        tag: u8,
        reference_kind: u8,
        reference_index: u16,
    },
    MethodTypeInfo {
        tag: u8,
        descriptor_index: u16,
    },
    InvokeDynamicInfo {
        tag: u8,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

pub struct ConstantPool {
    constant_pool_table: Vec<ConstantPoolEntry>
}


pub fn parse_constant_pool(class_bytes: &mut Vec<ClassByte>) -> ConstantPool {
    let cp_count = get_class_byte(class_bytes) - 1;
    let mut cp_table = Vec::with_capacity(cp_count as usize);

    let mut index = 1;

    while index <= cp_count {
        index += 1;
        let cp_tag = get_class_byte(class_bytes);

        cp_table.push(match cp_tag {
            CONSTANT_CLASS => ConstantPoolEntry::ClassInfo {
                tag: cp_tag,
                name_index: get_bytes_to_u16(class_bytes),
            },
            CONSTANT_FIELD_REF |
            CONSTANT_METHOD_REF |
            CONSTANT_INTERFACE_METHOD_REF => ConstantPoolEntry::RefInfo {
                tag: cp_tag,
                class_index: get_bytes_to_u16(class_bytes),
                name_and_type_index: get_bytes_to_u16(class_bytes),
            },
            CONSTANT_STRING => ConstantPoolEntry::StringInfo {
                tag: cp_tag,
                string_index: get_bytes_to_u16(class_bytes),
            },
            CONSTANT_INTEGER => ConstantPoolEntry::IntegerInfo {
                tag: cp_tag,
                value: get_bytes_to_i32(class_bytes),
            },
            CONSTANT_FLOAT => ConstantPoolEntry::FloatInfo {
                tag: cp_tag,
                value: get_bytes_to_f32(class_bytes),
            },
            //the 8 bytes take up 2 count
            CONSTANT_LONG => {
                index += 1;
                ConstantPoolEntry::LongInfo {
                    tag: cp_tag,
                    value: get_bytes_to_i64(class_bytes),
                }
            }
            CONSTANT_DOUBLE => {
                index += 1;
                ConstantPoolEntry::DoubleInfo {
                    tag: cp_tag,
                    value: get_bytes_to_f64(class_bytes),
                }
            }
            CONSTANT_NAME_AND_TYPE => ConstantPoolEntry::NameAndTypeInfo {
                tag: cp_tag,
                name_index: get_bytes_to_u16(class_bytes),
                descriptor_index: get_bytes_to_u16(class_bytes),
            },
            CONSTANT_UTF8 => ConstantPoolEntry::UTF8Info {
                tag: cp_tag,
                //u16->2bytes for the content bytes' length
                content: constant_pool_utf8_decode(get_class_bytes(get_bytes_to_u16(class_bytes) as usize, class_bytes)),

            },
            CONSTANT_METHOD_HANDLE => ConstantPoolEntry::MethodHandleInfo {
                tag: cp_tag,
                reference_kind: get_class_byte(class_bytes),
                reference_index: get_bytes_to_u16(class_bytes),
            },
            CONSTANT_METHOD_TYPE => ConstantPoolEntry::MethodTypeInfo {
                tag: cp_tag,
                descriptor_index: get_bytes_to_u16(class_bytes),
            },
            CONSTANT_INVOKE_DYNAMIC => ConstantPoolEntry::InvokeDynamicInfo {
                tag: cp_tag,
                bootstrap_method_attr_index: get_bytes_to_u16(class_bytes),
                name_and_type_index: get_bytes_to_u16(class_bytes),
            },
            _ => panic!("unsupported flag:{} for constant pool entry", cp_tag),
        });
    }
    ConstantPool {
        constant_pool_table: cp_table
    }
}